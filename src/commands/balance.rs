use chrono::{Local, NaiveDateTime};
use fancy::printcoln;
use diesel::SqliteConnection;

use crate::model::{
    blocked, entry,
    hours::{self, get_target},
};

pub fn execute(connection: &mut SqliteConnection) {
    let oldest_date = hours::get_oldest_date(connection);
    let now = Local::now().naive_local();

    match oldest_date {
        Some(oldest) => sum(connection, &oldest, &now),
        None => println!(
            "You need to add working hours using the 'set-hours' before you can get a balance!"
        ),
    }
}

fn sum(connection: &mut SqliteConnection, oldest: &NaiveDateTime, youngest: &NaiveDateTime) {
    let sum = get_sum(connection, oldest, youngest);
    let target = get_target(connection, oldest, youngest);

    let diff = target - sum;

    printcoln!("Tracked   | [blue]{:.2}[:]", sum);
    printcoln!("Target    | {:.2}", target);
    printcoln!("----------|----------");
    if diff > 0.0 {
        printcoln!("Behind    | [red|bold]{:.2}[:]", diff);
    } else {
        printcoln!("Ahead     | [green|bold]{:.2}[:]", diff);
    }
}

fn get_sum(
    connection: &mut SqliteConnection,
    oldest: &NaiveDateTime,
    youngest: &NaiveDateTime,
) -> f64 {
    let sum_clocked = entry::get_sum_for(oldest, youngest, connection);
    let sum_blocked = blocked::get_sum_for(oldest, youngest, connection);

    sum_clocked + sum_blocked as f64
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, NaiveDate, NaiveTime};

    use crate::db;

    use super::*;
    #[test]
    fn working_sum() {
        let mut connection = db::for_test();

        let oldest_day = NaiveDate::from_ymd(2022, 10, 1);
        let oldest_time = NaiveTime::from_hms(0, 0, 0);
        let oldest = NaiveDateTime::new(oldest_day, oldest_time);
        let first_day = oldest_day;
        let start = NaiveTime::from_hms(8, 0, 0);
        let lunch_start = NaiveTime::from_hms(12, 0, 0);
        let lunch_end = NaiveTime::from_hms(12, 30, 0);
        let end = NaiveTime::from_hms(16, 30, 0);

        let mut final_dt = oldest.clone() + Duration::days(7);

        let first_start = NaiveDateTime::new(first_day, start);
        let first_lunch = NaiveDateTime::new(first_day, lunch_start);
        let first_afternoon = NaiveDateTime::new(first_day, lunch_end);
        let first_end = NaiveDateTime::new(first_day, end);

        crate::set_hours::execute(&oldest_day, 36.0, &mut connection);
        crate::clock_in::execute(&Some(first_start), &mut connection);
        crate::clock_out::execute(&Some(first_lunch), &mut connection);

        let sum = get_sum(&mut connection, &oldest, &final_dt);
        assert_eq!(4.0, sum);

        crate::clock_in::execute(&Some(first_afternoon), &mut connection);
        crate::clock_out::execute(&Some(first_end), &mut connection);

        let sum = get_sum(&mut connection, &oldest, &final_dt);
        assert_eq!(8.0, sum);

        crate::block::execute(&first_day, 8.0, "test", &mut connection);
        let sum = get_sum(&mut connection, &oldest, &final_dt);
        assert_eq!(16.0, sum);

        let target = crate::model::hours::get_target(&mut connection, &oldest, &final_dt);
        assert_eq!(36.0, target);

        final_dt += Duration::days(5);
        let target = crate::model::hours::get_target(&mut connection, &oldest, &final_dt);
        assert_eq!(72.0, target);
    }
}
