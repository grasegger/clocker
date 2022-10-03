use crate::schema::*;
use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = hours)]
pub struct Hours {
    pub id: i32,
    pub beginning_with: NaiveDateTime,
    pub hours_per_week: u8
}

#[derive(Insertable)]
#[diesel(table_name = hours)]
pub struct NewHours<'a> {
    pub beginning_with: &'a NaiveDateTime,
    pub hours_per_week: &'a i32
}