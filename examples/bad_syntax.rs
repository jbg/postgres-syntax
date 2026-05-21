use postgres_syntax::sql;

fn main() {
    let _ = sql!("SELECT name, email_address FROM user WHERE id = $1 BLARG");
}
