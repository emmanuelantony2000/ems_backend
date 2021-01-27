use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tokio_postgres::types::ToSql;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub(super) name: String,
    pub(super) email: String,
    pub(super) password: String,
    pub(super) phno: String,
    pub(super) dob: NaiveDate,
    pub(super) role: String,
    pub(super) designation: String,
    pub(super) experience: i32,
    pub(super) address: String,
}

impl Employee {
    pub(super) fn params<'a>(&'a self, uuid: &'a Uuid) -> [&'a (dyn ToSql + Sync); 10] {
        [
            uuid as &(dyn ToSql + Sync),
            &self.name as &(dyn ToSql + Sync),
            &self.email as &(dyn ToSql + Sync),
            &self.password as &(dyn ToSql + Sync),
            &self.phno as &(dyn ToSql + Sync),
            &self.dob as &(dyn ToSql + Sync),
            &self.role as &(dyn ToSql + Sync),
            &self.designation as &(dyn ToSql + Sync),
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
    pub(super) password: String,
    pub(super) phno: String,
    pub(super) dob: NaiveDate,
    pub(super) role: String,
    pub(super) designation: String,
    pub(super) experience: i32,
    pub(super) address: String,
}
