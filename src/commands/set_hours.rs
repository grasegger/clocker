use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::model::hours;
use crate::model::hours::NewHours;

pub fn execute(when: &NaiveDate, hours_per_week: u8, connection: &mut SqliteConnection) {
    let existing = hours::existing(when, connection);

    match existing {
        Ok(id) => update(hours_per_week, id, connection),
        Err(_) => create(when, hours_per_week, connection),
    };

    println!(
        "Starting from {} you work {} hours per week",
        when, hours_per_week
    );
}

fn create(when: &NaiveDate, new_hours: u8, connection: &mut SqliteConnection) {
    use crate::schema::hours;
    let date = when.and_hms(0, 0, 0);

    let new_hours = NewHours {
        beginning_with: &date,
        hours_per_week: &(new_hours as i32),
    };

    diesel::insert_into(hours::table)
        .values(&new_hours)
        .execute(connection)
        .expect("Unable to set new hours.");
}

fn update(new_hours: u8, db_id: i32, connection: &mut SqliteConnection) {
    use crate::schema::hours::dsl::*;

    diesel::update(hours.filter(id.eq(db_id)))
        .set(hours_per_week.eq(new_hours as i32))
        .execute(connection)
        .expect("Unable to set new hours.");
}
