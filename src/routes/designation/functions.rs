use std::sync::Arc;

use tokio_postgres::types::{ToSql, Type};
use tokio_postgres::Client;
use uuid::Uuid;
use warp::{reject, reply, Rejection, Reply};

use super::Designation;
use crate::error::Error;

pub(super) async fn gd((id, db): (Uuid, Arc<Client>)) -> Result<impl Reply, Rejection> {
    let statement = db
        .prepare_typed(
            "SELECT D.NAME
            FROM EMPLOYEE E, DESIGNATION D
            WHERE E.DESIGNATION = D.ID
            AND E.ID = $1",
            &[Type::UUID],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;

    let row = db
        .query_one(&statement, &[&id])
        .await
        .map_err(|e| reject::custom(Error::QueryError(e)))?;

    Ok(reply::json(&Designation { name: row.get(0) }))
}

pub(super) async fn pd((name, db): (String, Arc<Client>)) -> Result<impl Reply, Rejection> {
    let statement = db
        .prepare_typed(
            "INSERT INTO DESIGNATION
            VALUES ($1, $2)",
            &[Type::UUID, Type::TEXT],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;

    let uuid = Uuid::new_v4();

    let res = db
        .execute(
            &statement,
            &[&uuid as &(dyn ToSql + Sync), &name as &(dyn ToSql + Sync)],
        )
        .await
        .map_err(|e| reject::custom(Error::ExecuteError(e)))?;

    if res != 1 {
        return Err(reject::custom(Error::InsertUnsuccessfulError(res, 1)));
    }

    Ok(reply::json(&uuid))
}

pub(super) async fn gda(db: Arc<Client>) -> Result<impl Reply, Rejection> {
    let statement = db
        .prepare_typed(
            "SELECT NAME
            FROM DESIGNATION",
            &[],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;

    let rows = db
        .query(&statement, &[])
        .await
        .map_err(|e| reject::custom(Error::QueryError(e)))?;
    let rows: Vec<String> = rows.iter().map(|row| row.get(0)).collect();

    Ok(reply::json(&rows))
}

pub(super) async fn pda(
    (designations, db): (Vec<String>, Arc<Client>),
) -> Result<impl Reply, Rejection> {
    let len = designations.len();

    let mut count = 1usize;
    let mut query = String::from("INSERT INTO DESIGNATION VALUES");

    for _ in 0..len {
        query.push_str(" (");

        query.push_str(format!("${}, ", count).as_str());
        count += 1;

        query.push_str(format!("${}),", count).as_str());
        count += 1;
    }
    query.pop();

    let types: Vec<Type> = [Type::UUID, Type::TEXT]
        .iter()
        .cycle()
        .cloned()
        .take(2 * len)
        .collect();

    let statement = db
        .prepare_typed(query.as_str(), &types)
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;

    let uuid: Vec<Uuid> = (0..len).map(|_| Uuid::new_v4()).collect();

    let designations: Vec<Designation> = designations
        .into_iter()
        .map(|name| Designation { name })
        .collect();
    let params: Vec<_> = designations
        .iter()
        .zip(uuid.iter())
        .flat_map(|(e, u)| e.params(u).to_vec().into_iter())
        .collect();
    let res = db
        .execute(&statement, &params[..])
        .await
        .map_err(|e| reject::custom(Error::ExecuteError(e)))?;

    if res != len as u64 {
        return Err(reject::custom(Error::InsertUnsuccessfulError(
            res,
            len as u64 - res,
        )));
    }

    Ok(reply::json(&uuid))
}
