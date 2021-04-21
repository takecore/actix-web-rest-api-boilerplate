use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use crate::db;
use crate::schema::users;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub company_id: i32, // TODO relation
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct CreateUser {
    pub company_id: i32,
    pub name: String,
}

impl User {
    pub fn all() -> Result<Vec<Self>, Error> {
        use crate::schema::users::dsl::users;
        let items = users.load::<Self>(&db::connect())?;
        Ok(items)
    }

    pub fn id(id: i32) -> Result<Option<Self>, Error> {
        use crate::schema::users::dsl::users;
        let item = users
            .find(id)
            .get_result::<Self>(&db::connect())
            .optional()?;
        Ok(item)
    }

    pub fn id_with_company_id(id: i32, company_id: i32) -> Result<Option<Self>, Error> {
        use crate::schema::users::dsl;
        let item = dsl::users
            .find(id)
            .filter(dsl::company_id.eq(company_id))
            .get_result::<Self>(&db::connect())
            .optional()?;
        Ok(item)
    }
}

impl CreateUser {
    pub fn create(&self) -> Result<User, Error> {
        use crate::schema::users::dsl::users;
        let item = diesel::insert_into(users)
            .values(self)
            .get_result::<User>(&db::connect())?;
        Ok(item)
    }
}
