use crate::schema::event;
use crate::schema::food;
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



#[derive(Clone, Debug, Serialize, PartialEq, Queryable)]
pub struct Food {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Insertable)]
#[table_name = "food"]
struct CreateFood<'a> {
    pub name: &'a str,
}

impl Food {
    pub fn insert<'a>(conn: &Conn, name_value: &'a str) -> Result<(), Box<dyn std::error::Error>> {
        use crate::schema::food::dsl::*;
        diesel::insert_into(food)
            .values(&CreateFood { name: name_value })
            .execute(conn)?;
        Ok(())
    }

    pub fn list(conn: &Conn) -> Result<Vec<Food>, Box<dyn std::error::Error>> {
        use crate::schema::food::dsl::*;
        Ok(food.load::<Food>(conn)?)
    }
}
