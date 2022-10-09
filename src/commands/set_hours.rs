use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::model::hours;
use crate::model::hours::NewHours;

pub fn execute(when: &NaiveDate, hours_per_week: f32, connection: &mut SqliteConnection) {
    let existing = hours::existing(when, connection);

    match existing {
        Ok(id) => update(hours_per_week, id, connection).unwrap(),
        Err(_) => create(when, hours_per_week, connection).unwrap(),
    };

    println!(
        "Starting from {} you work {} hours per week",
        when, hours_per_week
    );
}

fn create(
    when: &NaiveDate,
    new_hours: f32,
    connection: &mut SqliteConnection,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::hours;
    let date = when.and_hms(0, 0, 0);

    let new_hours = NewHours {
        beginning_with: &date,
        hours_per_week: &(new_hours),
    };

    diesel::insert_into(hours::table)
        .values(&new_hours)
        .execute(connection)
}

fn update(
    new_hours: f32,
    db_id: i32,
    connection: &mut SqliteConnection,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::hours::dsl::*;

    diesel::update(hours.filter(id.eq(db_id)))
        .set(hours_per_week.eq(new_hours))
        .execute(connection)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use chrono::NaiveDate;
    use diesel::dsl::count_star;

    #[test]
    fn insert_new_blocked_entry() {
        use crate::schema::hours::dsl::*;
        let mut connection = db::for_test();
        let when = NaiveDate::from_ymd(2022, 10, 22);
        let new_hours = 8.0;

        create(&when, new_hours, &mut connection).unwrap();

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
        let when = NaiveDate::from_ymd(2022, 10, 22);
        let new_hours = 8.0;

        create(&when, new_hours, &mut connection).unwrap();

        let existing = crate::model::hours::existing(&when, &mut connection);
        let expected = Ok(1);

        assert_eq!(expected, existing);
    }

    #[test]
    fn create_and_update() {
        use crate::schema::hours::dsl::*;

        let mut connection = db::for_test();
        let when = NaiveDate::from_ymd(2022, 10, 22);
        let frist_hours = 8.0;
        let second_hours = 16.0;

        execute(&when, frist_hours, &mut connection);
        execute(&when, second_hours, &mut connection);

        let count = hours.select(hours_per_week).first(&mut connection);

        assert_eq!(Ok(16.0 as f32), count);
    }
}
