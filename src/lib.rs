//! Compile-time syntax checking for PostgreSQL queries.
//!
//! `postgres-syntax` exposes the [`sql!`] macro, which parses a PostgreSQL
//! query string at compile time and returns the string unchanged when parsing
//! succeeds. If the query is not valid PostgreSQL syntax, compilation fails
//! with the parser error reported by [`pg_query`].
//!
//! This crate validates syntax only. It does not connect to a database, inspect
//! your schema, validate table or column names, or type-check query parameters.
//!
//! # Example
//!
//! ```
//! use postgres_syntax::sql;
//!
//! let query = sql!("SELECT name, email_address FROM users WHERE id = $1");
//! assert_eq!(
//!     query,
//!     "SELECT name, email_address FROM users WHERE id = $1",
//! );
//! ```
//!
//! Invalid SQL produces a compile error:
//!
//! ```compile_fail
//! use postgres_syntax::sql;
//!
//! let _ = sql!("SELECT * FROM users WHERE id = $1 BLARG");
//! ```
//!
//! [`pg_query`]: https://docs.rs/pg_query/

use litrs::StringLit;
use proc_macro::TokenStream;
use quote::quote;

/// Checks a PostgreSQL query for syntax errors at compile time.
///
/// The macro accepts exactly one string literal. When the string parses as
/// PostgreSQL SQL, the macro expands back to that same literal, so it can be
/// used anywhere a string literal would be accepted.
///
/// ```
/// use postgres_syntax::sql;
///
/// let query: &str = sql!("SELECT now()");
/// assert_eq!(query, "SELECT now()");
/// ```
///
/// Syntax errors are reported during compilation:
///
/// ```compile_fail
/// use postgres_syntax::sql;
///
/// let _ = sql!("SELECT FROM");
/// ```
///
/// The macro only checks PostgreSQL syntax. It does not verify object names,
/// parameter types, permissions, or any other database-specific semantics.
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let input = input.into_iter().collect::<Vec<_>>();
    if input.len() != 1 {
        let msg = format!(
            "expected exactly one string literal as input, got {} tokens",
            input.len()
        );
        return quote! { compile_error!(#msg) }.into();
    }
    let string_lit = match StringLit::try_from(&input[0]) {
        Ok(lit) => lit,
        Err(e) => return e.to_compile_error(),
    };

    if let Err(e) = pg_query::parse(string_lit.value()) {
        let msg = format!("failed to parse SQL query: {}", e);
        return quote! { compile_error!(#msg) }.into();
    }

    input.into_iter().collect()
}
