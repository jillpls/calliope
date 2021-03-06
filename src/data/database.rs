extern crate sqlx;
extern crate tokio;

use rocket_db_pools as rdp;
use rocket_db_pools::Database;

use sqlx::sqlite::{SqliteQueryResult, SqliteRow};
use sqlx::{migrate::MigrateDatabase, Connection, Error, Sqlite, SqliteConnection};

#[derive(Database)]
#[database("userdata")]
pub struct UserData(sqlx::SqlitePool);

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

pub async fn insert_user(
    login: &String,
    hash: &String,
    conn: &mut rdp::Connection<UserData>,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let query = "
insert into users
    ( login, hash, activated )
values
    ($1, $2, 1)";

    sqlx::query(&*query)
        .bind(login)
        .bind(hash)
        .execute(&mut **conn)
        .await
}

#[derive(sqlx::FromRow)]
pub struct LoginInfo {
    pub(crate) id: i32,
    pub(crate) hash: String,
    pub(crate) activated: bool,
}

pub async fn get_login_info(
    login: &String,
    conn: &mut rdp::Connection<UserData>,
) -> Result<LoginInfo, Error> {
    let query = "
    select id, hash, activated
    from users
    where login = $1
    ";
    sqlx::query_as::<_, LoginInfo>(&*query)
        .bind(login)
        .fetch_one(&mut **conn)
        .await
}

pub async fn update_user_session(
    id: &i32,
    session_id: &String,
    conn: &mut rdp::Connection<UserData>,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let query = "
    update users
    set session = $1
    where id = $2
    ";
    sqlx::query(&*query)
        .bind(session_id)
        .bind(id)
        .execute(&mut **conn)
        .await
}
#[cfg(test)]
mod tests {
    use super::connect;

    #[tokio::test]
    async fn test_connect() {
        connect(None).await.unwrap();
    }
}
