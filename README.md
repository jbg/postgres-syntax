# postgres-syntax

This crate provides simple compile-time syntax checking of PostgreSQL queries using the [pg_query](https://github.com/pganalyze/pg_query.rs) crate (currently version 6 which uses libpg_query 17).

It doesn't know anything about your schema, so it doesn't do type-checking or verify that tables you reference actually exist. It's just checking that the SQL syntax is valid. If you want type and name checking, you might prefer [sqlx](https://github.com/launchbadge/sqlx).

[![Crate](https://img.shields.io/crates/v/postgres-syntax.svg)](https://crates.io/crates/postgres-syntax)

# Example

The crate provides one macro, `postgres_syntax::sql`, which takes a string literal and passes it through if it's valid SQL syntax, or emits a compiler error if it's not valid SQL.

```rust
use postgres_syntax::sql;

let _ = sql!("SELECT name, email_address FROM user WHERE id = $1 BLARG");
```

```
error: failed to parse SQL query: syntax error at or near "BLARG"
 --> examples/bad_syntax.rs:4:11
  |
4 |   let _ = sql!("SELECT name, email_address FROM user WHERE id = $1 BLARG");
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
```

## License

`postgres-syntax` is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Any kinds of contributions are welcome as a pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in these crates by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgements

`postgres-syntax` development is sponsored by [AVStack](https://www.avstack.io/). We provide globally-distributed, scalable, managed A/V infrastructure.
