use crate::schema::*;
use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = hours)]
pub struct Hours {
    pub id: i32,
    pub beginning_with: NaiveDateTime,
    pub hours_per_week: f32,
}

#[derive(Insertable)]
#[diesel(table_name = hours)]
pub struct NewHours<'a> {
    pub beginning_with: &'a NaiveDateTime,
    pub hours_per_week: &'a f32,
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

pub fn get_oldest_date(connection: &mut SqliteConnection) -> NaiveDateTime {
    use crate::schema::hours::dsl::*;

    let mut oldest = Local::now().naive_local();

    let all = hours
        .load::<Hours>(connection)
        .expect("Unable to load hours from database");

    for hour in all {
        if oldest > hour.beginning_with {
            oldest = hour.beginning_with;
        }
    }

    oldest
}
