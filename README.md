# Postgres unrecognized node bug reproduction

# Quick start

1. Install Rust (recommended method: [use rustup](https://rustup.rs/)).
2. Clone the repo:

   ```
   git clone https://github.com/demurgos/pg_unrecognized_node.git
   cd pg_unrecognized_node
   ```

3. Create a fresh Postgres database
4. Fill the connection options in `src/main.rs`
5. Run it

   ```
   cargo run
   ```

# Description

This repository contains the best reproduction for what I believe is a memory
corruption error in Postgres' parser. Running this code reliably yields the
following parser error:

```
PgDatabaseError { severity: Error, code: "XX000", message: "unrecognized node type: 0", detail: None, hint: None, position: None, where: None, schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("parse_expr.c"), line: Some(373), routine: Some("transformExprRecurse") }
```

In particular, the node type in the message is not deterministic, suggesting
that it corresponds to a dirty memory location. The most common messages I see
are:

- `unrecognized node type: 0`
- `unrecognized node type: 123`

I mostly get the node types above but roughly 5% of the time I get other types such as:

- `unrecognized node type: 16`
- `unrecognized node type: 32`
- `unrecognized node type: 196608`
- `unrecognized node type: 16781666`
- `unrecognized node type: 16781674`
- `unrecognized node type: 16781682`
- `unrecognized node type: 16781690`
- `unrecognized node type: 16781698`
- `unrecognized node type: 16781706`

# Environment

- OS: Linux 5.12 (Arch Linux)
- Postgres: 13.3
- Arch: x86_64
