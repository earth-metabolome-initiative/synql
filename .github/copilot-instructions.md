# SynQL AI Developer Instructions

## Architecture & Big Picture

SynQL is a specialized tool for generating **modular Cargo workspaces** from SQL schemas. Unlike typical ORMs that generate a single monolithic crate, SynQL produces a **micro-crate architecture** where each table becomes its own crate.

### Core Crates
1.  **`synql`**: The code generator. Analyzes the schema and produces Rust code.
    *   **Goal**: Generate Type Safe Rust crates for each table.
    *   **Backend**: Targets `diesel` and `diesel-builders`.
2.  **`sql_relations`**: A semantic analysis layer over SQL foreign keys.
    *   **Key Concept**: Introspects "Same As" relationships (Vertical, Horizontal, Triangular) to model inheritance and diamond dependencies.

### Design Principles
*   **Parallel Compilation**: Table-per-crate allows maximum parallelism.
*   **Granular Dependencies**: Consumers import only what they need.
*   **Strict Correctness**: Enforces database consistency constraints (like triangular dependencies) at compile time via generated builders.

## Developer Workflows

### Testing
*   **Schema Simulation**: Use `sql_traits::prelude::ParserDB` to create in-memory databases from SQL strings for testing.
*   **Workspace Generation Tests**:
    *   Tests often involve generating a full workspace in a temporary directory (`tempfile::tempdir`).
    *   Example: `synql/tests/test_workspace_generation.rs`.
*   **Strict Lints**: The project forbids `missing_docs`, `broken_intra_doc_links`, etc. Run `cargo clippy` and `cargo doc` frequently.

### Build & Code Quality
*   **Clean Code**: `Cargo.toml` enforces strict lints. Ensure **every public item** has a doc comment.
*   **Formatting**: Use `cargo fmt`.

## Project Conventions

### Rust Patterns
*   **Prelude**: Prefer importing from `prelude` modules (`synql::prelude::*`, `sql_relations::prelude::*`).
*   **Error Handling**: Use `thiserror` for error types (`synql::structs::Error`).
*   **Traits**: Heavy use of traits (`TableLike`, `ColumnLike` from `sql-traits`) to abstract over different database backends (Postgres, ParserDB).

### Documentation
*   **Mandatory Docs**: All public functions and structs/traits **must** have documentation.
*   **Doc Tests**: Include usage examples in doc comments.
*   **No Unescaped Backticks**: `workspace.lints.rustdoc` forbids unescaped backticks.

## Domain Language (SQL Relations)

Understand the specific "Same As" terminology in `sql_relations`:
*   **Vertical Same As**: Inheritance-like, column in child maps to ancestor column.
*   **Horizontal Same As**: Sibling equivalence.
*   **Triangular Same As**: Diamond dependency consistency.

## Key Files & Directories
*   `synql/src/structs/synql.rs`: Main entry point for workspace generation (`SynQL` struct).
*   `sql_relations/src/traits/table_list_like.rs`: Example of semantic trait logic.
*   `synql/tests/`: Integration tests showing how to invoke the generator.
*   `Cargo.toml`: Source of truth for aggressive linting rules.
