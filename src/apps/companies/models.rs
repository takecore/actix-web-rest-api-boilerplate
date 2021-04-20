use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use crate::apps::companies::views::UpdateCompany as ViewUpdateCompany;
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
    pub fn all(connection: &PgConnection) -> Result<Vec<Self>, Error> {
        use crate::schema::companies::dsl::companies;
        let items = companies.load::<Self>(connection)?;
        Ok(items)
    }

    pub fn id(connection: &PgConnection, id: i32) -> Result<Option<Self>, Error> {
        use crate::schema::companies::dsl::companies;
        let item = companies
            .find(id)
            .get_result::<Self>(connection)
            .optional()?;
        Ok(item)
    }

    pub fn update(
        &self,
        connection: &PgConnection,
        update: &ViewUpdateCompany,
    ) -> Result<Company, Error> {
        let item = diesel::update(self)
            .set(UpdateCompany {
                name: update.name.as_ref().map(AsRef::as_ref),
            })
            .get_result::<Self>(connection)?;
        Ok(item)
    }

    pub fn delete(&self, connection: &PgConnection) -> Result<usize, Error> {
        let count = diesel::delete(self).execute(connection)?;
        Ok(count)
    }
}

impl CreateCompany {
    pub fn create(&self, connection: &PgConnection) -> Result<Company, Error> {
        use crate::schema::companies::dsl::companies;
        let item = diesel::insert_into(companies)
            .values(self)
            .get_result::<Company>(connection)?;
        Ok(item)
    }
}
