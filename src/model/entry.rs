use crate::schema::*;
use chrono::{prelude::*, Duration};
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = entries)]
pub struct Entry {
    pub id: i32,
    pub clock_in: NaiveDateTime,
    pub clock_out: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = entries)]
pub struct NewEntry<'a> {
    pub clock_in: &'a NaiveDateTime,
    pub clock_out: Option<&'a NaiveDateTime>,
}

pub fn running(connection: &mut SqliteConnection) -> Result<NaiveDateTime, diesel::result::Error> {
    use crate::schema::entries::dsl::*;
    entries
        .filter(clock_out.is_null())
        .select(clock_in)
        .first::<NaiveDateTime>(connection)
}

pub fn current(connection: &mut SqliteConnection) -> Result<i32, diesel::result::Error> {
    use crate::schema::entries::dsl::*;
    entries
        .filter(clock_out.is_null())
        .select(id)
        .first::<i32>(connection)
}

pub fn get_sum_for(
    oldest_date: &NaiveDateTime,
    now: &NaiveDateTime,
    connection: &mut SqliteConnection,
) -> f32 {
    use crate::schema::entries::dsl::*;

    let to_sum = entries
        .filter(clock_in.ge(oldest_date))
        .load::<Entry>(connection);

    let mut sum = 0.0;

    for mut entry_vec in to_sum {
        let entry = entry_vec.pop().unwrap();
        let duration: Duration = if let Some(out) = entry.clock_out {
            out - entry.clock_in
        } else {
            *now - entry.clock_in
        };
        sum += duration.num_hours() as f32;
        sum += duration.num_minutes() as f32 / 60.0;
        sum += duration.num_seconds() as f32 / 60.0 / 60.0;
    }

    sum
}
