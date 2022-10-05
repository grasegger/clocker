use chrono::{Duration, Local};
use diesel::SqliteConnection;

use crate::model::{blocked, entry, hours};

pub fn execute(connection: &mut SqliteConnection) {
    let oldest_date = hours::get_oldest_date(connection);
    println!("Summing entries starting from {}", oldest_date);

    let now = Local::now().naive_local();

    let sum_clocked = entry::get_sum_for(&oldest_date, &now, connection);
    let sum_blocked = blocked::get_sum_for(&oldest_date, &now, connection);

    let sum = sum_clocked + sum_blocked;
    println!("So far you tracked \n{:.2} hours.", sum)
}
