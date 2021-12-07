pub mod mysql;
pub mod pgsql;

pub enum DatabaseType {
    pgsql(pgsql::PostgresConnection),
    mysql(mysql::MySQLConnection),
}

pub struct NullConnection {
    hostname: String,
    port: i32,
    ssl: bool,
    tls: bool,
    user: String,
    password: String,
}

pub trait DatabaseConnection {
    fn new() -> Self;
    fn connect(&self) -> Result<(), &str> {
        Err("No Connection found.")
    }
}

impl DatabaseConnection for NullConnection {
    fn new() -> Self {
        Self {
            hostname: "".to_string(),
            port: 0,
            ssl: false,
            tls: true,
            user: "".to_string(),
            password: "".to_string(),
        }
    }
}
