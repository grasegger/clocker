use crate::schema::*;
use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
#[diesel(table_name = entries)]
pub struct Entry {
    pub id: i32,
    pub clock_in: NaiveDateTime,
    pub clock_out: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = entries)]
pub struct NewEntry {
    pub clock_in: NaiveDateTime,
    pub clock_out: Option<NaiveDateTime>,
}
