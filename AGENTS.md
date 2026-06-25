# AGENTS.md

Instructions for AI agents working in this repository.

## Project Summary

This repo contains exactly two XSD files (`schema.xsd`, `changeset.xsd`) and thin wrappers that ship them to two ecosystems:

- **Rust** (`src/lib.rs`) — embeds both XSDs as `pub const &str` via `include_str!`
- **Java/Maven** (`build.gradle.kts`) — bundles the XSD files as JAR classpath resources under `com.stano:schema-xsd`

There is no application logic here. All real work is editing the XSD files.

## Key Files

| File | Purpose |
|---|---|
| `schema.xsd` | Defines the full database structure format (`<database>`, tables, columns, keys, relations, views, etc.) |
| `changeset.xsd` | Defines the migration/changeset format (add/drop/rename operations) |
| `src/lib.rs` | Rust crate entry point — only exposes `SCHEMA_XSD` and `CHANGESET_XSD` constants |
| `build.gradle.kts` | Gradle build — packages XSDs into a JAR and configures Maven publishing |
| `Cargo.toml` | Rust crate metadata |
| `gradle.properties` | Maven group and version (`com.stano`, `1.0.0-SNAPSHOT`) |

## Build Commands

### Rust
```bash
cargo build          # compile
cargo test           # run tests
cargo publish        # publish to crates.io
```

### Java / Gradle
```bash
./gradlew build              # compile and package JAR
./gradlew publishToMavenLocal  # install to ~/.m2
./gradlew publish            # publish to staging-deploy/
./gradlew zipStagingDeploy   # zip staging artifacts for upload
```

## Making Changes

**To add or modify schema elements:** edit `schema.xsd` or `changeset.xsd` directly. Both build systems pick up changes automatically — no code changes needed unless the XSD file names change.

**To rename an XSD file:** update the filename, then update `src/lib.rs` (`include_str!` path), `build.gradle.kts` (`include("*.xsd")` covers any `.xsd`), and any documentation.

**Version bumps:** update `gradle.properties` (Java) and `Cargo.toml` (Rust) independently — they are published to separate registries.

## Conventions

- The XSD files use `lowerCamelCase` for attribute names and `kebab-case` for element names.
- Database-type-specific variants (H2, PostgreSQL, SQLite, SQL Server) are represented as separate optional child elements, not as attributes.
- Do not add runtime logic to `src/lib.rs` — it intentionally exposes only the raw XSD strings. Validation, parsing, and code generation belong in downstream crates/libraries.
