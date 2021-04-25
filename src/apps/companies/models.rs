use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use crate::db;
use crate::schema::companies;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "companies"]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "companies"]
pub struct CreateCompany {
    pub name: String,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "companies"]
pub struct UpdateCompany {
    pub name: Option<String>,
}

impl Company {
    pub fn all() -> Result<Vec<Self>, Error> {
        use crate::schema::companies::dsl::companies;
        let items = companies.load::<Self>(&db::connect())?;
        Ok(items)
    }

    pub fn id(id: i32) -> Result<Self, Error> {
        use crate::schema::companies::dsl::companies;
        let item = companies.find(id).get_result::<Self>(&db::connect())?;
        Ok(item)
    }

    pub fn create(create: &CreateCompany) -> Result<Company, Error> {
        use crate::schema::companies::dsl::companies;
        let item = diesel::insert_into(companies)
            .values(create)
            .get_result::<Company>(&db::connect())?;
        Ok(item)
    }

    pub fn update(&self, update: &UpdateCompany) -> Result<Company, Error> {
        let item = diesel::update(self)
            .set(update)
            .get_result::<Self>(&db::connect())?;
        Ok(item)
    }

    pub fn delete(&self) -> Result<usize, Error> {
        let count = diesel::delete(self).execute(&db::connect())?;
        Ok(count)
    }
}
