use std::env;

use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    SqliteConnection::establish(&db_url).unwrap()
}
