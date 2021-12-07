use postgres;

#[derive(Debug)]
pub struct PostgresConnection {
    hostname: String,
    port: i32,
    ssl: bool,
    tls: bool,
    user: String,
    password: String,
}