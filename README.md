# schema-xsd

XSD schema definitions for describing database structures.

## Overview

This library provides an XML Schema Definition (XSD) file that establishes a common vocabulary for database-as-code tooling:

- **`schema.xsd`** — defines a complete database structure (tables, columns, keys, relations, views, functions, procedures, enums, and vendor-specific SQL)

Supported databases: H2, PostgreSQL, SQLite, SQL Server.

The library is available as both a Maven artifact (XSDs bundled as JAR resources) and a Rust crate (XSD content embedded as compile-time string constants).

## Usage

### Java / Maven

**Gradle:**
```kotlin
implementation("com.stano:schema-xsd:1.0.0")
```

**Maven:**
```xml
<dependency>
    <groupId>com.stano</groupId>
    <artifactId>schema-xsd</artifactId>
    <version>1.0.0</version>
</dependency>
```

Access the XSD files from the JAR classpath:
```java
InputStream schema = getClass().getResourceAsStream("/schema.xsd");
```

### Rust

**`Cargo.toml`:**
```toml
[dependencies]
schema-xsd = "1.0.0"
```

**Usage:**
```rust
use schema_xsd::SCHEMA_XSD;

// Constant is &'static str containing the full XSD content
println!("{}", SCHEMA_XSD);
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

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your option.
