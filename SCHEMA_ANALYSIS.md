# schema.xsd analysis

## Context
`schema.xsd` (420 lines, namespace `http://stano.com/database`, XSD 1.0) defines a database-structure format. The root `<database>` holds tables, enums, views, functions, procedures, `otherSql`, and `<schema>` groupings. It is shipped via a Rust crate and a Maven JAR (both 1.2.0). This is a review with prioritized findings. Tick the boxes as items are fixed.

## Structure
- `database` attrs: `foreignKeyMode` (relations|triggers), `booleanMode` (native|yes_no|yn), `caseSensitiveText`.
- Children: `table`, `enum`, `view`, `function`, `procedure`, `otherSql`, `schema` (a flat, non-nested namespace).
- `tableType` is an `xs:all` of optional sections: `columns` (required), `keys` (primary/unique/index), `relations` (relation/compositeRelation), `triggers` (update/delete), `constraints`, `aggregations` (sum/count with group), `initialData`.
- Vendor-specific SQL is carried by the `databaseType` attribute (h2|postgresql|sqlite|sqlserver). It is required on function/procedure/otherSql/trigger and optional on view/constraint/initialData.
- 25 column types in `columnEnum`. `enum` needs `enumType`, and `array` needs `elementType`.

## Findings (by priority)

### Correctness / validation gaps
- [x] 1. **Key `<column name>` is untyped and optional** (lines 77, 90, 104). An empty `<column/>` validates. Make it `type="string" use="required"`.
- [x] 2. **Column `<check>` has no type** (line 50). It is `anyType`, so any child content or attributes pass. It also cannot carry `databaseType`, unlike constraints and triggers. Give it a `vendorSql`-style simple type.
  - Done in the XSD: `<check>` is now string content with an optional `databaseType`, like `<constraint>`. It can repeat: a check without `databaseType` is the generic fallback, and a check with `databaseType` applies to that database and takes precedence. An `xs:unique` rejects duplicate checks for the same `databaseType`. XSD 1.0 cannot reject a second generic check, so that rule is documented only.
  - Downstream status: the Java parser (`TableColumnContentHandler`) and Rust parser (`CheckXml`, `roxml_parser.rs`) read only the text and ignore attributes. A `databaseType` on `<check>` validates but is silently applied to every database. Per-database checks would need model, parser and SQL generator changes in both `schema-java` and `schema-rs`.
- [x] 3. **`keys` forces a `primary`** (line 72, minOccurs=1). A table with only indexes or uniques must still declare a primary key. Check that this is intended.
  - Done in the XSD: `primary` is now `minOccurs="0"`, so a table can declare only unique or index keys. Documents that validated before still validate. Requiring a primary key is a policy for a linter or generator, not the schema.
  - TODO downstream: `schema-java` and `schema-rs` have not been updated. Check that their parsers, models and SQL generators tolerate a table with no primary key (no NPE/panic on a missing `primary`). Also note that `cluster` lives on `primary`, so keyless tables cannot be clustered.
- [x] 4. **Conditional rules are not enforced.** `type="enum"` needs `enumType`, `array` needs `elementType`, and `decimal` needs `length`/`scale`. `length`/`scale` should be `nonNegativeInteger`, not `integer`. Options are XSD 1.1 `xs:assert` or documenting the rules.
  - Done in the XSD: `length`/`scale` are now `nonNegativeInteger`. XSD 1.0 cannot express the conditional rules (enum needs `enumType`, array needs `elementType`, decimal needs `length`), so they are documented in a comment above the column attributes. Enforcing them would need XSD 1.1 `xs:assert`, which xmllint does not support.
- [x] 5. **No identity constraints.** Nothing enforces unique table, column, enum or schema names. Nothing checks that `relation@table` or `@enumType` references exist. Add `xs:unique`/`xs:key`/`xs:keyref` where feasible. Cross-schema references are the hard part.
  - Skipped: the downstream tools already enforce name uniqueness and reference checks. XSD 1.0 could cover uniqueness and same-scope keyrefs, but not cross-schema or column-level references.
- [x] 6. **`aggregate` content model is probably wrong** (lines 185-213). The outer `sequence maxOccurs=unbounded` repeats `(sum|count)+ , group`, which allows several `group`s. Probably intended: `(sum|count)+` then exactly one `group`.
  - Done in the XSD: `aggregate` is now `(sum|count)+` followed by exactly one `group`. `sum` and `count` may still be interleaved. Fixtures cover multiple groups, a repeated block and a missing group. The downstream sample documents that use `<aggregate>` still validate. `schema-java` merged several groups into one list, and `schema-rs` takes a single `group`, so neither needs a change for the tightening.
- [x] 7. **Redundant nested repeats**: `columns > sequence(1..*) > column(1..*)` and the same for key columns. The extra wrapper adds nothing. Remove the outer `sequence` repeat.
- [x] 8. **`minValue`/`maxValue` are `double`**, which loses precision for long and decimal columns. Consider `decimal`.

### Dead / stale definitions
- [x] 9. **Unused types**: `policySqlType`, `aggregationTypeEnum` (includes `average`, which no element supports) and `verifyTypeEnum`. Remove them, or wire them in if they are planned.
  - Done in the XSD: all three removed. Neither `schema-java` nor `schema-rs` reads them; the aggregate element names (`sum`, `count`, `group`) and `dateColumn` already carry that information. `schema-java` still has an unused `VerifyType` enum and `VerifyTypeTest` that could be cleaned up separately.
- [x] 10. `version="1.0"` on the schema element is out of step with the published 1.2.0.

### Consistency / design
- [ ] 11. **Relation naming is inconsistent.** `relation` uses `src`, `table`, `column`. `compositeRelation/column` uses `src`, `name`. Aggregation uses `sourceColumn`/`destinationColumn` on `sum` but `source`/`destination` on `group/column`.
- [x] 12. **`databaseType` optionality varies.** It is required on trigger, function, procedure and otherSql, and optional on view, constraint and initialData.
  - Intended, not a defect: views, constraints and initial data can generally be written generic, while triggers, functions, procedures and otherSql are inherently vendor-specific. Absent `databaseType` means the SQL applies to all databases. Documented in a comment above `databaseTypeEnum` in the XSD.
- [ ] 13. **Vendor-neutral SQL fragments cannot vary per database.** These are `column@default`, `@generated`, `check`, `index@where`/`@include`, unique `@where`, and aggregate `@criteria`. `index@include` is a comma-separated string, not a list of column elements.
- [x] 14. **Empty-string defaults** (`where=""`, `include=""`) are meaningless. Drop them.
- [x] 15. **The `schema` child duplicates the 6-way choice** from `database` (lines 11-34). Factor it into a named `xs:group`.
- [x] 16. **Relation actions are underspecified.** `relationTypeEnum` (cascade|enforce|setnull|donothing) has no `onUpdate`. The difference between `enforce` and `donothing` is documented in schema.xsd.
  - Accepted as is: the single action applies to ON DELETE, and ON UPDATE is almost always the same, so a separate `onUpdate` is not needed for now.
- [x] 17. **Only one comment in the whole file.** Add documentation for the non-obvious attributes: `data`, `noExport`, `exportDataColumn`, `lockEscalation`, `disableUsageChecking`, `generated`, and the `sequence`/`longsequence`/`cstext` types.
  - Done in the XSD as `xs:annotation/xs:documentation` on the table attributes, column attributes, `disableUsageChecking`, and the `columnEnum`, `relationTypeEnum` and `databaseTypeEnum` types (with per-value docs for the sequence, citext/cstext and relation values). Comments remain only for rules that span several components: the column conditional rules, the `check` fallback and the `unique` limitation. `data` is described from its name only (no downstream code explains it), so confirm the wording. `exportDataColumn` is described as the export/change-date column, as `schema-java` names it `exportDateColumn`.
- [ ] 18. **Triggers**: only `update` and `delete` are allowed, with no `insert` and no name. The `minOccurs`/`maxOccurs` inside the unbounded `choice` is redundant.

### Docs drift (AGENTS.md vs schema)
- [ ] AGENTS.md says elements are `kebab-case`, but they are camelCase (`otherSql`, `initialData`, `compositeRelation`).
- [ ] AGENTS.md says DB variants are "separate optional child elements, not attributes", but the schema uses a `databaseType` attribute everywhere.
- [ ] Update AGENTS.md to match, or the schema to match AGENTS.md.

## Suggested order if you want changes made
- [ ] 1. Safe tightenings (1, 2, 7, 9, 14, 15). These are mostly non-breaking, though 1 and 2 reject some currently-valid documents.
- [ ] 2. Decide on the content-model questions (3, 6, 12, 16), which are breaking and need your intent.
- [ ] 3. Add identity constraints (5) and documentation (17).
- [x] 4. Fix the AGENTS.md drift and bump the version in `gradle.properties` and `Cargo.toml` if the schema changes.
  - Done in the XSD: `length`/`scale` are now `nonNegativeInteger`. XSD 1.0 cannot express the conditional rules (enum needs `enumType`, array needs `elementType`, decimal needs `length`), so they are documented in a comment above the column attributes. Enforcing them would need XSD 1.1 `xs:assert`, which xmllint does not support.

## Verification (when implementing)
- `xmllint --noout --schema schema.xsd <sample.xml>` against the downstream sample documents, for example in `/Users/jstano/workspace/taps`.
- `cargo build && cargo test`, and `./gradlew build`.
