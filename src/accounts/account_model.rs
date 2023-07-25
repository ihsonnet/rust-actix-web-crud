use crate::db;
use crate::error_handler::CustomError;
use crate::schema::accounts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "accounts"]
pub struct Account {
    pub href: String,
    pub description: String,
    pub name: String,
    pub base_type: String,
    pub schema_location: String,
    pub a_type: String,
    pub referred_type: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Accounts {
    pub id: i32,
    pub href: String,
    pub description: String,
    pub name: String,
    pub base_type: String,
    pub schema_location: String,
    pub a_type: String,
    pub referred_type: String,
}

impl Accounts {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let accounts = accounts::table.load::<Accounts>(&conn)?;
        Ok(accounts)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let account = accounts::table.filter(accounts::id.eq(id)).first(&conn)?;
        Ok(account)
    }

    pub fn create(account: Account) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let account = Account::from(account);
        let account = diesel::insert_into(accounts::table)
            .values(account)
            .get_result(&conn)?;
        Ok(account)
    }

    pub fn update( id: i32, account: Account) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let account = diesel::update(accounts::table)
            .filter(accounts::id.eq(id))
            .set(account)
            .get_result(&conn)?;
        Ok(account)
    }

    pub fn delete( id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(accounts::table.filter(accounts::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl Account {
    fn from(account: Account) -> Account {
        Account {
            href: account.href,
            description: account.description,
            name: account.name,
            base_type: account.base_type,
            schema_location: account.schema_location,
            a_type: account.a_type,
            referred_type: account.referred_type,
        }
    }
}



