use crate::schema::*;
use chrono::{prelude::*, Duration};
use diesel::prelude::*;

#[derive(Queryable, Debug)]
#[diesel(table_name = hours)]
pub struct Hours {
    pub id: i32,
    pub start_date: NaiveDateTime,
    pub stop_date: NaiveDateTime,
    pub mon: f32,
    pub tue: f32,
    pub wed: f32,
    pub thu: f32,
    pub fri: f32,
    pub sat: f32,
    pub sun: f32,
}

#[derive(Insertable)]
#[diesel(table_name = hours)]
pub struct NewHours<'a> {
    pub start_date: &'a NaiveDateTime,
    pub stop_date: &'a NaiveDateTime,
    pub mon: &'a f32,
    pub tue: &'a f32,
    pub wed: &'a f32,
    pub thu: &'a f32,
    pub fri: &'a f32,
    pub sat: &'a f32,
    pub sun: &'a f32,
}

pub fn existing(
    date: &NaiveDate,
    connection: &mut SqliteConnection,
) -> Result<i32, diesel::result::Error> {
    use crate::schema::hours::dsl::*;
    hours
        .filter(start_date.eq(date.and_hms(0, 0, 0)))
        .select(id)
        .first::<i32>(connection)
}

pub fn get_oldest_date(connection: &mut SqliteConnection) -> Option<NaiveDateTime> {
    use crate::schema::hours::dsl::*;

    let all = hours.load::<Hours>(connection);
    match all {
        Ok(all) => {
            let mut oldest = Some(all.first().clone().unwrap().start_date);

            for hour in all {
                if let Some(cmp) = oldest {
                    if cmp > hour.start_date {
                        oldest = Some(hour.start_date);
                    } else {
                        oldest = Some(hour.start_date);
                    }
                }
            }
            oldest
        }
        Err(_) => None,
    }
}

pub fn get_target(
    connection: &mut SqliteConnection,
    then: &NaiveDateTime,
    now: &NaiveDateTime,
) -> f64 {
    use crate::schema::hours::dsl::*;
    let to_sum = hours
        .filter(start_date.ge(then))
        .filter(start_date.lt(now))
        .load::<Hours>(connection)
        .unwrap();

    to_sum.iter().fold(0.0, |sum, value| {
        let mut date = value.start_date.clone();
        let mut out = 0.0;

        while &date <= now && date <= value.stop_date {
            match date.weekday() {
                Weekday::Mon => out += value.mon,
                Weekday::Tue => out += value.tue,
                Weekday::Wed => out += value.tue,
                Weekday::Thu => out += value.wed,
                Weekday::Fri => out += value.fri,
                Weekday::Sat => out += value.sat,
                Weekday::Sun => out += value.sun,
            };
            date += Duration::days(1);
        }

        sum + out as f64
    })
}
