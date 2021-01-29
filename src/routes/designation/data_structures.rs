use serde::{Deserialize, Serialize};
use tokio_postgres::types::ToSql;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Designation {
    pub(super) name: String,
}

impl Designation {
    pub(super) fn params<'a>(&'a self, uuid: &'a Uuid) -> [&'a (dyn ToSql + Sync); 2] {
        [
            uuid as &(dyn ToSql + Sync),
            &self.name as &(dyn ToSql + Sync),
        ]
    }
}
