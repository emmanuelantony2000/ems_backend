use std::sync::Arc;

use chrono::Utc;
use tokio_postgres::types::{ToSql, Type};
use tokio_postgres::Client;
use uuid::Uuid;
use warp::{reject, reply, Rejection, Reply};

use super::Attendance;
use crate::error::Error;

pub(super) async fn ga((employee_id, db): (Uuid, Arc<Client>)) -> Result<impl Reply, Rejection> {
    let statement = db
        .prepare_typed(
            "SELECT
            DATESTAMP, TIMESTAMP
            FROM ATTENDANCE
            WHERE EMPLOYEE_ID = $1",
            &[Type::UUID],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;

    let rows = db
        .query(&statement, &[&employee_id])
        .await
        .map_err(|e| reject::custom(Error::QueryError(e)))?;

    let rows: Vec<Attendance> = rows
        .iter()
        .map(|row| Attendance {
            date: row.get(0),
            time: row.get(1),
        })
        .collect();

    Ok(reply::json(&rows))
}

pub(super) async fn pa((employee_id, db): (Uuid, Arc<Client>)) -> Result<impl Reply, Rejection> {
    let id = Uuid::new_v4();
    let datetime = Utc::now().naive_utc();
    let date = datetime.date();
    let time = datetime.time();

    let statement = db
        .prepare_typed(
            "INSERT INTO ATTENDANCE
            VALUES ($1, $2, $3, $4)",
            &[Type::UUID, Type::UUID, Type::DATE, Type::TIME],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;

    let res = db
        .execute(
            &statement,
            &[
                &id as &(dyn ToSql + Sync),
                &employee_id as &(dyn ToSql + Sync),
                &date as &(dyn ToSql + Sync),
                &time as &(dyn ToSql + Sync),
            ],
        )
        .await
        .map_err(|e| {
            if let Some(x) = e.code() {
                if x.code() == "23505" {
                    return reject::custom(Error::DuplicateAttendanceError);
                }
            }
            reject::custom(Error::ExecuteError(e))
        })?;

    if res != 1 {
        return Err(reject::custom(Error::InsertUnsuccessfulError(res, 1)));
    }

    Ok(reply::reply())
}
