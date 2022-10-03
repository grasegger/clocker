use std::env;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;

use crate::model::entry::NewEntry;

mod model;
mod schema;

#[derive(Parser, Debug)]
#[command(author,version, about, long_about = None)]
stuct Args{}

#[derive(Subcommand)]
enum Commands {
    In {
        #[arg(short, long)]
        start: Option<NaiveDateTime>
    },
    Out {
        #[arg(short, long)]
        end: Option<NaiveDateTime>
    },
    Csv {},
    Balance {},
}
fn main() {
    println!("Hello, world!");
    let mut connection = establish_connection();

    let new_entry = NewEntry {
        clock_in: NaiveDateTime::default(),
        clock_out: None,
    };

    diesel::insert_into(crate::schema::entries::table)
        .values(&new_entry)
        .execute(&mut connection);
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    SqliteConnection::establish(&db_url).unwrap()
}
