use std::convert::Infallible;
use std::sync::Arc;

use tokio_postgres::types::Type;
use tokio_postgres::Client;
use uuid::Uuid;
use warp::http;
use warp::reply;
use warp::Reply;

use super::{Employee, EmployeeId};
use crate::auth::generate_password;

macro_rules! bail {
    ($res:ident) => {
        match $res {
            Ok(x) => x,
            Err(e) => {
                return Ok(reply::with_status(
                    format!("{:?}", e),
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response());
            }
        };
    };
}

pub(super) async fn ge((id, db): (Uuid, Arc<Client>)) -> Result<impl Reply, Infallible> {
    let statement = db
        .prepare_typed(
            "SELECT
            NAME, EMAIL, PHNO, DOB, ROLE, DESIGNATION, EXPERIENCE, ADDRESS
            FROM EMPLOYEE
            WHERE ID = $1",
            &[Type::UUID],
        )
        .await;
    let statement = bail!(statement);

    let row = db.query_one(&statement, &[&id]).await;
    let row = bail!(row);

    Ok(reply::json(&Employee {
        name: row.get(0),
        email: row.get(1),
        password: String::new(),
        phno: row.get(2),
        dob: row.get(3),
        role: row.get(4),
        designation: row.get(5),
        experience: row.get(6),
        address: row.get(7),
    })
    .into_response())
}

pub(super) async fn ges(db: Arc<Client>) -> Result<impl Reply, Infallible> {
    let statement = db
        .prepare_typed(
            "SELECT *
            FROM EMPLOYEE",
            &[],
        )
        .await;
    let statement = bail!(statement);

    let rows = db.query(&statement, &[]).await;
    let rows = bail!(rows);

    let rows: Vec<EmployeeId> = rows
        .iter()
        .map(|row| EmployeeId {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            password: row.get(3),
            phno: row.get(4),
            dob: row.get(5),
            role: row.get(6),
            designation: row.get(7),
            experience: row.get(8),
            address: row.get(9),
        })
        .collect();

    Ok(reply::json(&rows).into_response())
}

pub(super) async fn pe(
    (mut employees, db): (Vec<Employee>, Arc<Client>),
) -> Result<impl Reply, Infallible> {
    let len = employees.len();

    let mut count = 1usize;
    let mut query = String::from("INSERT INTO EMPLOYEE VALUES");

    for _ in 0..len {
        query.push_str(" (");

        for _ in 0..9 {
            query.push_str(format!("${}, ", count).as_str());
            count += 1;
        }

        query.push_str(format!("${}),", count).as_str());
        count += 1;
    }
    query.pop();

    let types: Vec<Type> = [
        Type::UUID,
        Type::TEXT,
        Type::TEXT,
        Type::TEXT,
        Type::TEXT,
        Type::DATE,
        Type::TEXT,
        Type::TEXT,
        Type::INT4,
        Type::TEXT,
    ]
    .iter()
    .cycle()
    .cloned()
    .take(9 * len)
    .collect();

    let statement = db.prepare_typed(query.as_str(), &types).await;
    let statement = bail!(statement);

    let uuid: Vec<Uuid> = (0..len).map(|_| Uuid::new_v4()).collect();

    employees
        .iter_mut()
        .zip(uuid.iter())
        .for_each(|(e, u)| e.password = generate_password(&e.password, u));

    let params: Vec<_> = employees
        .iter()
        .zip(uuid.iter())
        .flat_map(|(e, u)| e.params(u).to_vec().into_iter())
        .collect();
    let res = db.execute(&statement, &params[..]).await;
    let res = bail!(res);

    if res != len as u64 {
        return Ok(reply::with_status(
            "Insert Unsuccessful",
            http::StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response());
    }

    Ok(reply::json(&uuid).into_response())
}
