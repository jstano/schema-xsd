# AGENTS.md

Instructions for AI agents working in this repository.

## Project Summary

This repo contains exactly one XSD file (`schema.xsd`) and thin wrappers that ship it to two ecosystems:

- **Rust** (`src/lib.rs`) — embeds the XSD as a `pub const &str` via `include_str!`
- **Java/Maven** (`build.gradle.kts`) — bundles the XSD file as a JAR classpath resource under `com.stano:schema-xsd`

There is no application logic here. All real work is editing the XSD file.

## Key Files

| File | Purpose |
|---|---|
| `schema.xsd` | Defines the full database structure format (`<database>`, tables, columns, keys, relations, views, etc.) |
| `src/lib.rs` | Rust crate entry point — only exposes the `SCHEMA_XSD` constant |
| `build.gradle.kts` | Gradle build — packages the XSD into a JAR and configures Maven publishing |
| `Cargo.toml` | Rust crate metadata |
| `gradle.properties` | Maven group and version (`com.stano`, `1.2.0`) |

## Build Commands

### Rust
```bash
cargo build          # compile
cargo test           # validate tests/fixtures against schema.xsd (needs xmllint)
cargo publish        # publish to crates.io
```

### Java / Gradle
```bash
./gradlew build              # compile and package JAR
./gradlew publishToMavenLocal   # install to ~/.m2
./gradlew publishToMavenCentral # publish to Maven Central
```

## Testing

`tests/validate.rs` runs `xmllint --schema schema.xsd` over every document in `tests/fixtures/valid` (must validate) and `tests/fixtures/invalid` (must be rejected). When you add or change a schema rule, add a fixture to the matching directory.

## Making Changes

**To add or modify schema elements:** edit `schema.xsd` directly. Both build systems pick up changes automatically — no code changes needed unless the XSD file names change.

**To rename an XSD file:** update the filename, then update `src/lib.rs` (`include_str!` path), `build.gradle.kts` (`include("*.xsd")` covers any `.xsd`), and any documentation.

**Version bumps:** update `gradle.properties` (Java) and `Cargo.toml` (Rust) independently — they are published to separate registries.

## Conventions

- The XSD files use `lowerCamelCase` for attribute names and `kebab-case` for element names.
- Database-type-specific variants (H2, PostgreSQL, SQLite, SQL Server) are represented as separate optional child elements, not as attributes.
- Do not add runtime logic to `src/lib.rs` — it intentionally exposes only the raw XSD strings. Validation, parsing, and code generation belong in downstream crates/libraries.
