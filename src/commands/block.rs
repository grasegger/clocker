use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::model::blocked;
use crate::model::blocked::NewBlocked;

pub fn execute(from: &NaiveDate, hours: f32, reason: &str, connection: &mut SqliteConnection) {
    let existing = blocked::existing(&from, connection);

    match existing {
        Ok(id) => update(id, hours, reason, connection).unwrap(),
        Err(_) => create(&from, hours, reason, connection).unwrap(),
    };
}

fn create(
    from: &NaiveDate,
    new_hours: f32,
    reason: &str,
    connection: &mut SqliteConnection,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::blocked;
    let new_date = from.and_hms(0, 0, 0);

    let data = NewBlocked {
        date: &new_date,
        reason,
        hours: &(new_hours),
    };

    diesel::insert_into(blocked::table)
        .values(&data)
        .execute(connection)
}

fn update(
    db_id: i32,
    new_hours: f32,
    new_reason: &str,
    connection: &mut SqliteConnection,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::blocked::dsl::*;

    diesel::update(blocked.filter(id.eq(db_id)))
        .set((hours.eq(new_hours), reason.eq(new_reason)))
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
        use crate::schema::blocked::dsl::*;
        let mut connection = db::for_test();
        let when = NaiveDate::from_ymd(2022, 10, 22);
        let new_hours = 8.0;

        create(&when, new_hours, "test", &mut connection).unwrap();

        let count = blocked.select(count_star()).first(&mut connection);

        assert_eq!(Ok(1), count);
    }

    #[test]
    fn check_non_existing() {
        let mut connection = db::for_test();
        let when = NaiveDate::from_ymd(2022, 10, 22);

        let existing = crate::model::blocked::existing(&when, &mut connection);
        let expected = Err(diesel::result::Error::NotFound);

        assert_eq!(expected, existing);
    }

    #[test]
    fn existing() {
        let mut connection = db::for_test();
        let when = NaiveDate::from_ymd(2022, 10, 22);
        let new_hours = 8.0;

        create(&when, new_hours, "test", &mut connection).unwrap();

        let existing = crate::model::blocked::existing(&when, &mut connection);
        let expected = Ok(1);

        assert_eq!(expected, existing);
    }

    #[test]
    fn create_and_update() {
        use crate::schema::blocked::dsl::*;

        let mut connection = db::for_test();
        let when = NaiveDate::from_ymd(2022, 10, 22);
        let frist_hours = 8.0;
        let second_hours = 16.0;

        execute(&when, frist_hours, "test", &mut connection);
        execute(&when, second_hours, "test", &mut connection);

        let count = blocked.select(hours).first(&mut connection);

        assert_eq!(Ok(16.0 as f32), count);
    }
}
