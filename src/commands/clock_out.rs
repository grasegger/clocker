use chrono::{DateTime, Local, NaiveDateTime};
use diesel::prelude::*;

use crate::model::entry::{self, NewEntry};

pub fn execute(when: &Option<NaiveDateTime>, connection: &mut SqliteConnection) {
    match entry::running(connection) {
        Ok(result) => stop(when, connection),
        Err(_) => println!("There is no active session, so I can't clock you out."),
    };
}

fn stop(when: &Option<NaiveDateTime>, connection: &mut SqliteConnection) {
    use crate::schema::entries::dsl::*;

    let end = match when {
        Some(when) => *when,
        None => {
            let now: DateTime<Local> = Local::now();
            now.naive_local()
        }
    };

    let current = entry::current(connection).unwrap();

    diesel::update(entries.filter(id.eq(current)))
        .set(clock_out.eq(end))
        .execute(connection)
        .expect("Unable to execute clock out in database.");

    println!("Clocked you out at {}", end);
}
