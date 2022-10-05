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

pub fn get_sum_for(
    oldest_date: &NaiveDateTime,
    now: &NaiveDateTime,
    connection: &mut SqliteConnection,
) -> f32 {
    use crate::schema::blocked::dsl::*;

    let to_sum = blocked
        .filter(date.ge(oldest_date))
        .load::<Blocked>(connection);

    let mut sum = 0.0;

    for mut entry_vec in to_sum {
        let entry = entry_vec.pop().unwrap();
        sum += entry.hours;
    }

    sum
}
