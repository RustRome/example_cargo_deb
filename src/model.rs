use crate::schema::event;
use diesel;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use diesel::RunQueryDsl;
use serde_derive::{Deserialize, Serialize};

pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone, Debug, Serialize, PartialEq, Queryable)]
pub struct Event {
    pub id: i32,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Insertable)]
#[table_name = "event"]
struct CreateEvent<'a> {
    pub title: &'a str,
}

impl Event {
    pub fn insert<'a>(conn: &Conn, title_value: &'a str) -> Result<(), Box<dyn std::error::Error>> {
        use crate::schema::event::dsl::*;
        diesel::insert_into(event)
            .values(&CreateEvent { title: title_value })
            .execute(conn)?;
        Ok(())
    }

    pub fn list(conn: &Conn) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        use crate::schema::event::dsl::*;
        Ok(event.load::<Event>(conn)?)
    }
}
