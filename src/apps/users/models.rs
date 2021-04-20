use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub company_id: i32, // TODO relation
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl User {
    pub fn all(connection: &PgConnection) -> Result<Vec<Self>, Error> {
        use crate::schema::users::dsl::users;
        let items = users.load::<Self>(connection)?;
        Ok(items)
    }

    pub fn id(connection: &PgConnection, id: i32) -> Result<Option<Self>, Error> {
        use crate::schema::users::dsl::users;
        let item = users.find(id).get_result::<Self>(connection).optional()?;
        Ok(item)
    }
}
