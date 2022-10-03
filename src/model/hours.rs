use crate::schema::*;
use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = hours)]
pub struct Hours {
    pub id: i32,
    pub beginning_with: NaiveDateTime,
    pub hours_per_week: u8,
}

#[derive(Insertable)]
#[diesel(table_name = hours)]
pub struct NewHours<'a> {
    pub beginning_with: &'a NaiveDateTime,
    pub hours_per_week: &'a i32,
}

pub fn existing(
    date: &NaiveDate,
    connection: &mut SqliteConnection,
) -> Result<i32, diesel::result::Error> {
    use crate::schema::hours::dsl::*;
    hours
        .filter(beginning_with.eq(date.and_hms(0, 0, 0)))
        .select(id)
        .first::<i32>(connection)
}
