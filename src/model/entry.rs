use crate::schema::*;
use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = entries)]
pub struct Entry {
    pub id: i32,
    pub clock_in: NaiveDateTime,
    pub clock_out: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = entries)]
pub struct NewEntry<'a> {
    pub clock_in: &'a NaiveDateTime,
    pub clock_out: Option<&'a NaiveDateTime>,
}
