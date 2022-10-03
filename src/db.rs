use std::env;

use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    let mut connection = SqliteConnection::establish(&db_url).unwrap();

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Unable to run migrations on database.");
    connection
}
