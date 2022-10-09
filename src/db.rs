use std::env;

use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn establish_connection(url: Option<String>) -> SqliteConnection {
    dotenv().ok();

    let db_url = if let Some(url) = url {
        url
    } else {
        env::var("DATABASE_URL").unwrap()
    };
    let mut connection = SqliteConnection::establish(&db_url).unwrap();

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Unable to run migrations on database.");
    connection
}

#[cfg(test)]
pub fn for_test() -> SqliteConnection {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(10000000..99999999);

    let mut connection = establish_connection(Some(format!("{}.db", num)));
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Unable to run migrations on database.");
    connection
}
