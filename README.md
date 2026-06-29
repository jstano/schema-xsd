# schema-xsd

XSD schema definitions for describing database structures and incremental migrations.

## Overview

This library provides two XML Schema Definition (XSD) files that establish a common vocabulary for database-as-code tooling:

- **`schema.xsd`** — defines a complete database structure (tables, columns, keys, relations, views, functions, procedures, enums, and vendor-specific SQL)
- **`changeset.xsd`** — defines incremental migration operations (add/drop/rename for tables, columns, keys, constraints, relations, and views)

Supported databases: H2, PostgreSQL, SQLite, SQL Server.

The library is available as both a Maven artifact (XSDs bundled as JAR resources) and a Rust crate (XSD content embedded as compile-time string constants).

## Usage

### Java / Maven

**Gradle:**
```kotlin
implementation("com.stano:schema-xsd:1.0.0-SNAPSHOT")
```

**Maven:**
```xml
<dependency>
    <groupId>com.stano</groupId>
    <artifactId>schema-xsd</artifactId>
    <version>1.0.0-SNAPSHOT</version>
</dependency>
```

Access the XSD files from the JAR classpath:
```java
InputStream schema    = getClass().getResourceAsStream("/schema.xsd");
```

### Rust

**`Cargo.toml`:**
```toml
[dependencies]
schema-xsd = "1.0.0"
```

**Usage:**
```rust
use schema_xsd::{SCHEMA_XSD, CHANGESET_XSD};

// Both constants are &'static str containing the full XSD content
println!("{}", SCHEMA_XSD);
println!("{}", CHANGESET_XSD);
```

## Schema Overview

### `schema.xsd`

Defines a `<database>` root element containing:

- **Tables** — columns (with types, lengths, defaults, generated values, enum references), primary keys, unique constraints, indexes, foreign key relations, triggers, named constraints, and aggregations
- **Column types** — sequence, byte, short, int, long, float, double, decimal, boolean, date, time, datetime, timestamp, timestamptz, char, varchar, enum, text, citext, cstext, binary, uuid, json, array
- **Views** — named SQL views with per-database-type variants
- **Functions / Procedures** — vendor-specific SQL implementations
- **Enums** — custom enumeration types with values and codes
- **Vendor SQL** — arbitrary SQL blocks run at the top or bottom of generated scripts

### `changeset.xsd`

Defines a `<changeset>` root element containing ordered migration operations:

- `add-table` / `drop-table` / `rename-table`
- `add-column` / `drop-column` / `rename-column` / `modify-column`
- `add-key` / `drop-key`
- `add-constraint` / `drop-constraint`
- `add-relation` / `drop-relation`
- `add-view` / `drop-view`

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your option.
