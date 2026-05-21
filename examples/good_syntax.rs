use postgres_syntax::sql;

fn main() {
    let query = sql!("SELECT name, email_address FROM user WHERE id = $1");
    assert_eq!(query, "SELECT name, email_address FROM user WHERE id = $1");
}
