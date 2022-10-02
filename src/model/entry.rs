use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = "in_out")]
pub struct Entry {
    pub id: i64,
    pub clock_in: DateTime<Local>,
    pub clock_out: Option<DateTime<Local>>,
}
