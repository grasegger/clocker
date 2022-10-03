use chrono::{Duration, Local};
use diesel::SqliteConnection;

use crate::model::{blocked, entry, hours};

pub fn execute(connection: &mut SqliteConnection) {
    let oldest_date = hours::get_oldest_date(connection);
    println!("{}", oldest_date);

    let now = Local::now().naive_local();

    let sum_clocked = entry::getSumFor(&oldest_date, &now, connection);
    let sum_blocked = blocked::getSumFor(&oldest_date, &now, connection);

    let sum = sum_clocked + sum_blocked;
    println!("{:.2}", sum)
}
