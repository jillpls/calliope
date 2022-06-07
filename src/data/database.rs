extern crate sqlx;
extern crate tokio;

use sqlx::{migrate::MigrateDatabase, Connection, Sqlite, SqliteConnection};

pub async fn connect(path: Option<&str>) -> Box<Result<SqliteConnection, sqlx::Error>> {
    let url = if let Some(p) = path {
        format!("sqlite://{}", p)
    } else {
        "sqlite::memory".to_string()
    };
    if !Sqlite::database_exists(&url).await.unwrap_or(false) {
        Sqlite::create_database(&url).await.unwrap();
    }
    Box::from(SqliteConnection::connect(&url).await)
}

#[cfg(test)]
mod tests {
    use super::connect;

    #[tokio::test]
    async fn test_connect() {
        connect(None).await.unwrap();
    }
}
