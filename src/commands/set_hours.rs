use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::model::hours;
use crate::model::hours::NewHours;

pub fn execute(
    start: &NaiveDate,
    stop: &NaiveDate,
    mon: f32,
    tue: f32,
    wed: f32,
    thu: f32,
    fri: f32,
    sat: f32,
    sun: f32,
    connection: &mut SqliteConnection,
) {
    let existing = hours::existing(start, connection);

    match existing {
        Ok(id) => update(id, stop, mon, tue, wed, thu, fri, sat, sun, connection).unwrap(),
        Err(_) => create(start, stop, mon, tue, wed, thu, fri, sat, sun, connection).unwrap(),
    };

    println!(
        "Starting from {} you work {} hours per week",
        start,
        mon + tue + wed + thu + fri + sat + sun
    );
}

fn create(
    start: &NaiveDate,
    stop: &NaiveDate,
    mon: f32,
    tue: f32,
    wed: f32,
    thu: f32,
    fri: f32,
    sat: f32,
    sun: f32,
    connection: &mut SqliteConnection,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::hours;
    let start_date = start.and_hms(0, 0, 0);
    let stop_date = stop.and_hms(0, 0, 0);

    let new_hours = NewHours {
        start_date: &start_date,
        stop_date: &stop_date,
        mon: &mon,
        tue: &tue,
        wed: &wed,
        thu: &thu,
        fri: &fri,
        sat: &sat,
        sun: &sun,
    };

    diesel::insert_into(hours::table)
        .values(&new_hours)
        .execute(connection)
}

fn update(
    db_id: i32,
    stop: &NaiveDate,
    new_mon: f32,
    new_tue: f32,
    new_wed: f32,
    new_thu: f32,
    new_fri: f32,
    new_sat: f32,
    new_sun: f32,
    connection: &mut SqliteConnection,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::hours::dsl::*;
    let new_stop_date = stop.and_hms(0, 0, 0);

    diesel::update(hours.filter(id.eq(db_id)))
        .set((
            mon.eq(&new_mon),
            tue.eq(&new_tue),
            wed.eq(&new_wed),
            thu.eq(&new_thu),
            fri.eq(&new_fri),
            sat.eq(&new_sat),
            sun.eq(&new_sun),
            stop_date.eq(new_stop_date),
        ))
        .execute(connection)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use chrono::{Duration, NaiveDate};
    use diesel::dsl::count_star;

    #[test]
    fn insert_new__entry() {
        use crate::schema::hours::dsl::*;
        let mut connection = db::for_test();
        let start = NaiveDate::from_ymd(2022, 10, 22);
        let stop = start.clone() + Duration::days(7);

        execute(
            &start,
            &stop,
            8.0,
            8.0,
            8.0,
            8.0,
            8.0,
            8.0,
            8.0,
            &mut connection,
        );

        let count = hours.select(count_star()).first(&mut connection);

        assert_eq!(Ok(1), count);
    }

    #[test]
    fn check_non_existing() {
        let mut connection = db::for_test();
        let when = NaiveDate::from_ymd(2022, 10, 22);

        let existing = crate::model::hours::existing(&when, &mut connection);
        let expected = Err(diesel::result::Error::NotFound);

        assert_eq!(expected, existing);
    }

    #[test]
    fn existing() {
        let mut connection = db::for_test();
        let start = NaiveDate::from_ymd(2022, 10, 22);
        let stop = start.clone() + Duration::days(14);
        let new_hours = 8.0;

        execute(
            &start,
            &stop,
            8.0,
            8.0,
            8.0,
            8.0,
            8.0,
            8.0,
            8.0,
            &mut connection,
        );

        let existing = crate::model::hours::existing(&start, &mut connection);
        let expected = Ok(1);

        assert_eq!(expected, existing);
    }

    #[test]
    fn create_and_update() {
        use crate::schema::hours::dsl::*;

        let mut connection = db::for_test();
        let start = NaiveDate::from_ymd(2022, 10, 22);
        let stop = start.clone() + Duration::days(14);
        let second_hours = 16.0;

        execute(
            &start,
            &stop,
            8.0,
            8.0,
            8.0,
            8.0,
            8.0,
            8.0,
            8.0,
            &mut connection,
        );
        execute(
            &start,
            &stop,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            &mut connection,
        );

        let result = hours.select(mon).first(&mut connection);

        assert_eq!(Ok(1.0 as f32), result);
    }
}
