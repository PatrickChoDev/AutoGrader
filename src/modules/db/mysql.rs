pub struct MySQLConnection {
    hostname: String,
    port: i32,
    ssl: bool,
    tls: bool,
    user: String,
    password: String,
}