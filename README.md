# Postgres unrecognized node bug reproduction

# Quick start

Requires Rust, [get Rust with Rustup](https://rustup.rs/).

```
git clone https://github.com/demurgos/pg_unrecognized_node.git
cd pg_unrecognized_node
# Edit the connection options in `src/main.rs`
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
s
- `unrecognized node type: 16`
- `unrecognized node type: 32`

# Environment

- OS: Linux 5.12 (Arch Linux)
- Postgres: 13.3
- Arch: x86_64
