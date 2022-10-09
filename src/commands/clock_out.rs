use chrono::{DateTime, Local, NaiveDateTime};
use diesel::prelude::*;

use crate::model::entry;

pub fn execute(when: &Option<NaiveDateTime>, connection: &mut SqliteConnection) {
    match entry::running(connection) {
        Ok(_result) => {
            stop(when, connection).unwrap();
        }
        Err(_) => {
            println!("There is no active session, so I can't clock you out.");
        }
    };
}

fn stop(
    when: &Option<NaiveDateTime>,
    connection: &mut SqliteConnection,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::entries::dsl::*;

    let end = match when {
        Some(when) => *when,
        None => {
            let now: DateTime<Local> = Local::now();
            now.naive_local()
        }
    };

    let current = entry::current(connection).unwrap();

    println!("Clocking you out at {}", end);

    diesel::update(entries.filter(id.eq(current)))
        .set(clock_out.eq(end))
        .execute(connection)
}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::db;
    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn stop_current() {
        let mut connection = db::for_test();
        let d = NaiveDate::from_ymd(2022, 10, 22);
        let t = NaiveTime::from_hms(13, 13, 13);

        let when = Some(NaiveDateTime::new(d, t));
        crate::clock_in::create(&when, &mut connection).unwrap();

        let stop_result = stop(&None, &mut connection);
        assert_eq!(Ok(1), stop_result);

        let running = entry::running(&mut connection);
        let error = Err(diesel::result::Error::NotFound);

        assert_eq!(error, running);
    }
}
