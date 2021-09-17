# compile-time-create-file

[![Documentation][docs-badge]][docs-link]
[![crates.io][crate-badge]][crate-link]
[![CI status][ci-badge]][ci-link]

Create files and directories at compile time using a procedural macro in Rust.

## Example

```rust
use compile_time_create_file::create_file;

create_file!(
    "migrations/users.sql",
    "create table if not exists users (
    id serial,
    username varchar(128) not null,
    password varchar(512) not null,
    email varchar(256) not null,
    enabled boolean not null default true
);
"
);
```

## Install

Add `compile-time-create-file = "0.1.0"` to your development
dependencies:

```toml
[dev-dependencies]
compile-time-create-file = "0.1.0"
```

[docs-badge]: https://img.shields.io/docsrs/compile-time-create-file
[docs-link]: https://docs.rs/compile-time-create-file
[crate-badge]: https://img.shields.io/crates/v/compile-time-create-file
[crate-link]: https://crates.io/crates/compile-time-create-file
[ci-badge]: https://img.shields.io/github/workflow/status/mondeja/compile-time-create-file/CI?label=tests
[ci-link]: https://github.com/mondeja/compile-time-create-file/actions
