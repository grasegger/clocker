use chrono::{DateTime, Local, NaiveDateTime};
use diesel::prelude::*;

use crate::model::entry::{self, NewEntry};

pub fn execute(when: &Option<NaiveDateTime>, connection: &mut SqliteConnection) {
    match entry::running(connection) {
        Ok(result) => {
            println!("There is already a clock running, started on {}", result);
        }
        Err(_) => {
            create(when, connection).unwrap();
        }
    };
}

pub(crate) fn create(
    when: &Option<NaiveDateTime>,
    connection: &mut SqliteConnection,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::entries;

    let start = match when {
        Some(when) => *when,
        None => {
            let now: DateTime<Local> = Local::now();
            now.naive_local()
        }
    };

    let new_entry = NewEntry {
        clock_in: &start,
        clock_out: None,
    };

    diesel::insert_into(entries::table)
        .values(new_entry)
        .execute(connection)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::db;
    use crate::schema::entries::dsl::*;
    use chrono::{NaiveDate, NaiveTime};
    use diesel::dsl::count_star;

    #[test]
    fn insert_new_entry() {
        let mut connection = db::for_test();
        let d = NaiveDate::from_ymd(2022, 10, 22);
        let t = NaiveTime::from_hms(13, 13, 13);

        let when = Some(NaiveDateTime::new(d, t));
        create(&when, &mut connection).unwrap();

        let count = entries.select(count_star()).first(&mut connection);

        assert_eq!(Ok(1), count);
    }

    #[test]
    fn no_running_task() {
        let mut connection = db::for_test();
        let running = entry::running(&mut connection);
        let expeted = Err(diesel::result::Error::NotFound);
        assert_eq!(expeted, running);
    }

    #[test]
    fn running_task() {
        let mut connection = db::for_test();
        let d = NaiveDate::from_ymd(2022, 10, 22);
        let t = NaiveTime::from_hms(13, 13, 13);

        let when = Some(NaiveDateTime::new(d, t));
        create(&when, &mut connection).unwrap();

        let running = entry::running(&mut connection);
        let expected = Ok(when.unwrap());

        assert_eq!(expected, running);
    }
}
