use crate::schema::*;
use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = blocked)]
pub struct Blocked {
    pub id: i32,
    pub date: NaiveDateTime,
    pub reason: String,
    pub hours: f32,
}

#[derive(Insertable)]
#[diesel(table_name = blocked)]
pub struct NewBlocked<'a> {
    pub date: &'a NaiveDateTime,
    pub reason: &'a str,
    pub hours: &'a f32,
}

pub fn existing(
    target_date: &NaiveDate,
    connection: &mut SqliteConnection,
) -> Result<i32, diesel::result::Error> {
    use crate::schema::blocked::dsl::*;

    blocked
        .filter(date.eq(target_date.and_hms(0, 0, 0)))
        .select(id)
        .first::<i32>(connection)
}
