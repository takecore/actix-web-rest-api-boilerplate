use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use crate::apps::companies::views::UpdateCompany as ViewUpdateCompany;
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

#[derive(Debug, Insertable)]
#[table_name = "companies"]
pub struct CreateCompany {
    pub name: String,
}

#[derive(AsChangeset)]
#[table_name = "companies"]
pub struct UpdateCompany<'a> {
    pub name: Option<&'a str>,
}

impl Company {
    pub fn all() -> Result<Vec<Self>, Error> {
        use crate::schema::companies::dsl::companies;
        let items = companies.load::<Self>(&db::connect())?;
        Ok(items)
    }

    pub fn id(id: i32) -> Result<Option<Self>, Error> {
        use crate::schema::companies::dsl::companies;
        let item = companies
            .find(id)
            .get_result::<Self>(&db::connect())
            .optional()?;
        Ok(item)
    }

    pub fn update(&self, update: &ViewUpdateCompany) -> Result<Company, Error> {
        let item = diesel::update(self)
            .set(UpdateCompany {
                name: update.name.as_ref().map(AsRef::as_ref),
            })
            .get_result::<Self>(&db::connect())?;
        Ok(item)
    }

    pub fn delete(&self) -> Result<usize, Error> {
        let count = diesel::delete(self).execute(&db::connect())?;
        Ok(count)
    }
}

impl CreateCompany {
    pub fn create(&self) -> Result<Company, Error> {
        use crate::schema::companies::dsl::companies;
        let item = diesel::insert_into(companies)
            .values(self)
            .get_result::<Company>(&db::connect())?;
        Ok(item)
    }
}
