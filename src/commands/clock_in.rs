use chrono::{DateTime, Local, NaiveDateTime};
use diesel::prelude::*;

use crate::model::entry::{self, NewEntry};

pub fn execute(when: &Option<NaiveDateTime>, connection: &mut SqliteConnection) {
    match entry::running(connection) {
        Ok(result) => {
            println!("There is already a clock running, started on {}", result)
        }
        Err(_) => create(when, connection),
    };
}

fn create(when: &Option<NaiveDateTime>, connection: &mut SqliteConnection) {
    use crate::schema::entries;

    let start = match when {
        Some(when) => *when,
        None => {
            let now: DateTime<Local> = Local::now();
            now.naive_local()
        }
    };

    let new_entry = NewEntry {
        clock_in: &start,
        clock_out: None,
    };

    diesel::insert_into(entries::table)
        .values(new_entry)
        .execute(connection)
        .expect("Unable to execute clock in in database.");
}
