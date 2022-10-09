use std::ops::Sub;

use crate::schema::*;
use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
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

pub fn get_oldest_date(connection: &mut SqliteConnection) -> Option<NaiveDateTime> {
    use crate::schema::hours::dsl::*;

    let all = hours.load::<Hours>(connection);
    match all {
        Ok(all) => {
            let mut oldest = Some(all.first().clone().unwrap().beginning_with);

            for hour in all {
                if let Some(cmp) = oldest {
                    if cmp > hour.beginning_with {
                        oldest = Some(hour.beginning_with);
                    } else {
                        oldest = Some(hour.beginning_with);
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
    let duration = now.sub(*then);
    let mut weeks = duration.num_weeks();

    if duration.num_days() > 7 {
        weeks += 1;
    }

    let to_sum = hours
        .filter(beginning_with.ge(then))
        .filter(beginning_with.lt(now))
        .load::<Hours>(connection)
        .unwrap();

    to_sum.iter().fold(0.0, |sum, value| {
        sum + value.hours_per_week as f64 * weeks as f64
    })
}
