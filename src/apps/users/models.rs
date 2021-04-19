use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub company_id: u64,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl User {
    pub fn all(connection: &MysqlConnection) -> Result<Vec<User>, Error> {
        use crate::schema::users::dsl::users;
        let items = users.load::<User>(connection)?;
        Ok(items)
    }

    pub fn id(connection: &MysqlConnection, id: u64) -> Result<Option<User>, Error> {
        use crate::schema::users::dsl::users;
        let item = users.find(id).get_result::<User>(connection).optional()?;
        Ok(item)
    }
}
