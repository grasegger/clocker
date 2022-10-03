use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::model::blocked;
use crate::model::blocked::NewBlocked;

pub fn execute(from: &NaiveDate, hours: f32, reason: &str, connection: &mut SqliteConnection) {
    let existing = blocked::existing(&from, connection);

    match existing {
        Ok(id) => update(id, hours, reason, connection),
        Err(_) => create(&from, hours, reason, connection),
    }
}

fn create(from: &NaiveDate, new_hours: f32, reason: &str, connection: &mut SqliteConnection) {
    use crate::schema::blocked;
    let new_date = from.and_hms(0, 0, 0);

    let data = NewBlocked {
        date: &new_date,
        reason,
        hours: &(new_hours),
    };

    diesel::insert_into(blocked::table)
        .values(&data)
        .execute(connection)
        .expect("Unable to set blocked hours.");
}

fn update(db_id: i32, new_hours: f32, new_reason: &str, connection: &mut SqliteConnection) {
    use crate::schema::blocked::dsl::*;

    diesel::update(blocked.filter(id.eq(db_id)))
        .set((hours.eq(new_hours), reason.eq(new_reason)))
        .execute(connection)
        .expect("Unable to set new blocked hours.");
}
