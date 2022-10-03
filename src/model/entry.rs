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

pub fn running(connection: &mut SqliteConnection) -> Result<NaiveDateTime, diesel::result::Error> {
    use crate::schema::entries::dsl::*;
    entries
        .filter(clock_out.is_null())
        .select(clock_in)
        .first::<NaiveDateTime>(connection)
}

pub fn current(connection: &mut SqliteConnection) -> Result<i32, diesel::result::Error> {
    use crate::schema::entries::dsl::*;
    entries
        .filter(clock_out.is_null())
        .select(id)
        .first::<i32>(connection)
}
