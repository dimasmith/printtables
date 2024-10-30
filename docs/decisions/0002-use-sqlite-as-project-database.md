# ADR-0002: Use sqlite database and sqlx library for db operations 

| Code | Status | Date |
|:--- |:--- |:--- |
| `ADR-0002` | Accepted | 2024-10-29 |

## Context

Printtables is a personal, single-user, self-hosted Rust application designed for managing 3D-printing tasks. The goal of the project is to provide a lightweight, easy-to-use solution that requires minimal setup and maintenance. Given the nature of the project—focused on a single user and self-hosting—selecting a suitable database management system (DBMS) is crucial. The primary requirements for the DBMS are simplicity, minimal installation steps, low maintenance overhead, and compatibility with the Rust ecosystem.

## Decision

SQLite was chosen as the database, and sqlx was selected as the Rust library for interacting with SQLite.

### Rationale

The decision to use SQLite with sqlx is based on the following considerations:

**Simplicity of Installation and Setup**:

SQLite is a serverless, self-contained, zero-configuration, and transactional SQL database engine. This makes it an ideal choice for a self-hosted application where ease of setup is a priority. SQLite does not require a separate database server to be installed, configured, and maintained, unlike more complex systems like PostgreSQL or MySQL. The database is just a file that is created when the application is first run, making the installation process as simple as downloading and running the application.

**Low Maintenance Overhead**:

Since SQLite does not have a server component, there is no need for database administration tasks such as server monitoring, user management, or performing regular backups on a separate server. All data is stored in a single file on the host machine, which can be easily backed up, copied, and moved as needed. This significantly reduces the operational overhead, especially for a single-user application.

**Lightweight and Suitable for Single-User Applications**:

SQLite is designed for lightweight use cases and works well with single-user applications where concurrent access and complex queries are minimal. For software intended for individual use, the lack of concurrent multi-user support is not a limiting factor. SQLite’s small footprint (less than 1 MB) aligns well with the lightweight nature of the application.

**Integration with Rust Ecosystem using sqlx**:

sqlx is a modern SQL toolkit for Rust that supports async, compile-time checked SQL queries, and integrates seamlessly with SQLite. It offers a strong type system and compile-time query validation, making it a safer and more efficient choice compared to rusqlite. By leveraging sqlx, the project benefits from:

- Asynchronous operations, which are better suited for a modern Rust application.
- Compile-time checked queries, reducing runtime errors and improving code safety.
- A flexible API that supports raw SQL, providing more control over database operations.

**Sufficient for the Application's Data Requirements**:

The data requirements are relatively simple. The system need to store projects, tables, assemblies, and parts. All those items are relatively simple.

### Consequences

**Positive Consequences**:

- Simple setup and deployment process for end users.
- No need for ongoing database server management, reducing the operational burden.
- Lower risk of security vulnerabilities associated with running a separate database server.
- Easy to backup, migrate, or share the data file.
- Efficient use of resources (memory, disk space).
- Asynchronous database operations and compile-time query checks with sqlx lead to better performance and safety.

**Negative Consequences**:

- Limited scalability if the project scope expands to require multi-user support or high-concurrency features.
- Direct file access may pose risks of corruption if not properly handled in environments with unexpected shutdowns or crashes.

Alternatives Considered

**PostgreSQL**:

A powerful, open-source relational database management system with advanced features. Rejected due to higher setup complexity, additional dependencies, and management overhead not justified for a single-user application.

**MySQL/MariaDB**:

Another popular relational database option that supports multi-user access and scalability. Rejected for similar reasons as PostgreSQL, including higher complexity in installation and maintenance.

**File-based Storage (e.g., JSON, CSV)**:

Simpler storage mechanisms like JSON or CSV files could have been used for simplicity. However, these options lack the ACID compliance and query capabilities provided by SQLite, making them less reliable and more cumbersome to manage over time.

**rusqlite**:

An alternative Rust library for interfacing with SQLite. Initially considered but later replaced by sqlx due to its synchronous nature and less flexibility compared to sqlx, which provides a more modern async and compile-time safe approach.

## Conclusion

SQLite, coupled with sqlx, is the best fit for the Printtables project due to its simplicity, minimal installation, low maintenance requirements, asynchronous operations, and compile-time query safety. The decision aligns with the goals of the Printtables project to remain lightweight, easy to use, and easy to deploy.
