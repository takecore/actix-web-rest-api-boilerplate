use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use crate::apps::companies::views::SearchQuery;
use crate::db::connection;
use crate::db::pagination::Paginate;
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
        let items = companies.load::<Self>(&connection::connect())?;
        Ok(items)
    }

    pub fn search(params: SearchQuery) -> Result<(Vec<Self>, i64), Error> {
        let mut query = companies::table.order(companies::id.desc()).into_boxed();

        if let Some(v) = params.name {
            query = query.filter(companies::name.like(format!("%{}%", v)))
        }

        let (items, total_pages) = query
            .paginate(params.page.unwrap_or(1))
            .load_and_count_pages::<Self>(&connection::connect())?;
        Ok((items, total_pages))
    }

    pub fn id(id: i32) -> Result<Self, Error> {
        use crate::schema::companies::dsl::companies;
        let item = companies
            .find(id)
            .get_result::<Self>(&connection::connect())?;
        Ok(item)
    }

    pub fn create(create: &CreateCompany) -> Result<Self, Error> {
        use crate::schema::companies::dsl::companies;
        let item = diesel::insert_into(companies)
            .values(create)
            .get_result::<Company>(&connection::connect())?;
        Ok(item)
    }

    pub fn update(&self, update: &UpdateCompany) -> Result<Self, Error> {
        let item = diesel::update(self)
            .set(update)
            .get_result::<Self>(&connection::connect())?;
        Ok(item)
    }

    pub fn delete(&self) -> Result<usize, Error> {
        let count = diesel::delete(self).execute(&connection::connect())?;
        Ok(count)
    }
}
