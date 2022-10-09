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
) -> f64 {
    use crate::schema::entries::dsl::*;

    let to_sum = entries
        .filter(clock_in.ge(oldest_date))
        .load::<Entry>(connection)
        .unwrap();

    to_sum.iter().fold(0.0, |sum, value| {
        let duration: Duration = if let Some(out) = value.clock_out {
            out - value.clock_in
        } else {
            *now - value.clock_in
        };
        sum + duration.num_hours() as f64 + ((duration.num_minutes() % 60) as f64 / 60.0)
    })
}
