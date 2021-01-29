use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tokio_postgres::types::ToSql;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub(super) name: String,
    pub(super) email: String,
    pub(super) phone_number: String,
    pub(super) dob: NaiveDate,
    pub(super) role: String,
    pub(super) designation: String,
    pub(super) experience: i32,
    pub(super) address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeePassword {
    pub(super) name: String,
    pub(super) email: String,
    pub(super) password: String,
    pub(super) phone_number: String,
    pub(super) dob: NaiveDate,
    pub(super) role: String,
    pub(super) designation: String,
    pub(super) experience: i32,
    pub(super) address: String,
}

impl EmployeePassword {
    pub(super) fn params<'a>(
        &'a self,
        uuid: &'a Uuid,
        designation: &'a Uuid,
    ) -> [&'a (dyn ToSql + Sync); 10] {
        [
            uuid as &(dyn ToSql + Sync),
            &self.name as &(dyn ToSql + Sync),
            &self.email as &(dyn ToSql + Sync),
            &self.password as &(dyn ToSql + Sync),
            &self.phone_number as &(dyn ToSql + Sync),
            &self.dob as &(dyn ToSql + Sync),
            &self.role as &(dyn ToSql + Sync),
            designation as &(dyn ToSql + Sync),
            &self.experience as &(dyn ToSql + Sync),
            &self.address as &(dyn ToSql + Sync),
        ]
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeId {
    pub(super) id: Uuid,
    pub(super) name: String,
    pub(super) email: String,
    pub(super) phone_number: String,
    pub(super) dob: NaiveDate,
    pub(super) role: String,
    pub(super) designation: String,
    pub(super) experience: i32,
    pub(super) address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeOptionAdmin {
    pub(super) id: Uuid,
    pub(super) name: Option<String>,
    pub(super) password: Option<String>,
    pub(super) phone_number: Option<String>,
    pub(super) role: Option<String>,
    pub(super) designation: Option<String>,
    pub(super) experience: Option<i32>,
    pub(super) address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeOptionUser {
    pub(super) password: Option<String>,
    pub(super) phone_number: Option<String>,
    pub(super) address: Option<String>,
}
