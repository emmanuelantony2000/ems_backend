use std::sync::Arc;

use tokio_postgres::types::{ToSql, Type};
use tokio_postgres::Client;
use uuid::Uuid;
use warp::{reject, reply, Rejection, Reply};

use super::{Employee, EmployeeId, EmployeeOptionAdmin, EmployeeOptionUser, EmployeePassword};
use crate::auth::generate_password;
use crate::error::Error;

pub(super) async fn ge((id, db): (Uuid, Arc<Client>)) -> Result<impl Reply, Rejection> {
    let statement = db
        .prepare_typed(
            "SELECT
            E.NAME, E.EMAIL, E.PHONE_NUMBER, E.DOB, E.ROLE, D.NAME, E.EXPERIENCE, E.ADDRESS
            FROM EMPLOYEE E, DESIGNATION D
            WHERE E.ID = $1
            AND D.ID = E.DESIGNATION",
            &[Type::UUID],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;

    let row = db
        .query_one(&statement, &[&id])
        .await
        .map_err(|e| reject::custom(Error::QueryError(e)))?;

    Ok(reply::json(&Employee {
        name: row.get(0),
        email: row.get(1),
        phone_number: row.get(2),
        dob: row.get(3),
        role: row.get(4),
        designation: row.get(5),
        experience: row.get(6),
        address: row.get(7),
    }))
}

pub(super) async fn pe(
    (mut employee, db): (EmployeePassword, Arc<Client>),
) -> Result<impl Reply, Rejection> {
    let statement = db
        .prepare_typed(
            "INSERT INTO EMPLOYEE
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
            &[
                Type::UUID,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::DATE,
                Type::TEXT,
                Type::UUID,
                Type::INT4,
                Type::TEXT,
            ],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;

    let uuid = Uuid::new_v4();

    let designation_statement = db
        .prepare_typed(
            "SELECT ID
            FROM DESIGNATION
            WHERE NAME = $1",
            &[Type::TEXT],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;

    let row = db
        .query_one(&designation_statement, &[&employee.designation])
        .await
        .map_err(|e| reject::custom(Error::QueryError(e)))?;
    let designation_id: Uuid = row.get(0);

    employee.password = generate_password(&employee.password, &uuid);

    let res = db
        .execute(&statement, &employee.params(&uuid, &designation_id))
        .await
        .map_err(|e| reject::custom(Error::ExecuteError(e)))?;

    if res != 1 {
        return Err(reject::custom(Error::InsertUnsuccessfulError(res, 1)));
    }

    Ok(reply::json(&uuid))
}

pub(super) async fn pea(
    (
        EmployeeOptionAdmin {
            id,
            name,
            password,
            phone_number,
            role,
            designation,
            experience,
            address,
        },
        db,
    ): (EmployeeOptionAdmin, Arc<Client>),
) -> Result<impl Reply, Rejection> {
    tracing::debug!("Entered pea");

    let statement = db
        .prepare_typed(
            "SELECT NAME PASSWORD PHONE_NUMBER ROLE DESIGNATION EXPERIENCE ADDRESS
            FROM EMPLOYEE
            WHERE ID = $1",
            &[Type::UUID],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;
    let row = db
        .query_one(&statement, &[&id])
        .await
        .map_err(|e| reject::custom(Error::QueryError(e)))?;

    let statement = db
        .prepare_typed(
            "UPDATE EMPLOYEE
            SET NAME = $1, PASSWORD = $2, PHONE_NUMBER = $3, ROLE = $4, DESIGNATION = $5,
            EXPERIENCE = $6, ADDRESS = $7
            WHERE ID = $8",
            &[
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::INT4,
                Type::TEXT,
                Type::UUID,
            ],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;

    let designation_id: Uuid = if let Some(designation) = designation {
        let designation_statement = db
            .prepare_typed(
                "SELECT ID
                FROM DESIGNATION
                WHERE NAME = $1",
                &[Type::TEXT],
            )
            .await
            .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;
        let designation_row = db
            .query_one(&designation_statement, &[&designation])
            .await
            .map_err(|e| reject::custom(Error::QueryError(e)))?;
        designation_row.get(0)
    } else {
        row.get(4)
    };

    let res = db
        .execute(
            &statement,
            &[
                &name.unwrap_or(row.get(0)) as &(dyn ToSql + Sync),
                &password
                    .as_ref()
                    .map(|password| generate_password(password, &id))
                    .unwrap_or(row.get(1)) as &(dyn ToSql + Sync),
                &phone_number.unwrap_or(row.get(2)) as &(dyn ToSql + Sync),
                &role.unwrap_or(row.get(3)) as &(dyn ToSql + Sync),
                &designation_id as &(dyn ToSql + Sync),
                &experience.unwrap_or(row.get(5)) as &(dyn ToSql + Sync),
                &address.unwrap_or(row.get(6)) as &(dyn ToSql + Sync),
                &id as &(dyn ToSql + Sync),
            ],
        )
        .await
        .map_err(|e| reject::custom(Error::ExecuteError(e)))?;

    if res != 1 {
        return Err(reject::custom(Error::InsertUnsuccessfulError(res, 1)));
    }

    Ok(reply())
}

pub(super) async fn peu(
    (
        EmployeeOptionUser {
            password,
            phone_number,
            address,
        },
        id,
        db,
    ): (EmployeeOptionUser, Uuid, Arc<Client>),
) -> Result<impl Reply, Rejection> {
    tracing::debug!("Entered peu");

    let statement = db
        .prepare_typed(
            "SELECT PASSWORD PHONE_NUMBER ADDRESS
            FROM EMPLOYEE
            WHERE ID = $1",
            &[Type::UUID],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;
    let row = db
        .query_one(&statement, &[&id])
        .await
        .map_err(|e| reject::custom(Error::QueryError(e)))?;

    let statement = db
        .prepare_typed(
            "UPDATE EMPLOYEE
            SET PASSWORD = $1,
            PHONE_NUMBER = $2,
            ADDRESS = $3
            WHERE ID = $4",
            &[Type::TEXT, Type::TEXT, Type::TEXT, Type::UUID],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;

    let res = db
        .execute(
            &statement,
            &[
                &password
                    .as_ref()
                    .map(|password| generate_password(password, &id))
                    .unwrap_or(row.get(0)) as &(dyn ToSql + Sync),
                &phone_number.unwrap_or(row.get(1)) as &(dyn ToSql + Sync),
                &address.unwrap_or(row.get(2)) as &(dyn ToSql + Sync),
                &id as &(dyn ToSql + Sync),
            ],
        )
        .await
        .map_err(|e| reject::custom(Error::ExecuteError(e)))?;

    if res != 1 {
        return Err(reject::custom(Error::InsertUnsuccessfulError(res, 1)));
    }

    Ok(reply())
}

pub(super) async fn ges(db: Arc<Client>) -> Result<impl Reply, Rejection> {
    let statement = db
        .prepare_typed(
            "SELECT
            E.ID, E.NAME, E.EMAIL, E.PHONE_NUMBER, E.DOB, E.ROLE, D.NAME, E.EXPERIENCE, E.ADDRESS
            FROM EMPLOYEE E, DESIGNATION D
            WHERE D.ID = E.DESIGNATION",
            &[],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;

    let rows = db
        .query(&statement, &[])
        .await
        .map_err(|e| reject::custom(Error::QueryError(e)))?;

    let rows: Vec<EmployeeId> = rows
        .iter()
        .map(|row| EmployeeId {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            phone_number: row.get(3),
            dob: row.get(4),
            role: row.get(5),
            designation: row.get(6),
            experience: row.get(7),
            address: row.get(8),
        })
        .collect();

    Ok(reply::json(&rows))
}

pub(super) async fn pes(
    (mut employees, db): (Vec<EmployeePassword>, Arc<Client>),
) -> Result<impl Reply, Rejection> {
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
        Type::UUID,
        Type::INT4,
        Type::TEXT,
    ]
    .iter()
    .cycle()
    .cloned()
    .take(10 * len)
    .collect();

    let statement = db
        .prepare_typed(query.as_str(), &types)
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;

    let uuid: Vec<Uuid> = (0..len).map(|_| Uuid::new_v4()).collect();

    let designation_statement = db
        .prepare_typed(
            "SELECT ID
            FROM DESIGNATION
            WHERE NAME = $1",
            &[Type::TEXT],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;
    let mut designations: Vec<Uuid> = Vec::with_capacity(len);

    for employee in employees.iter() {
        let row = db
            .query_one(&designation_statement, &[&employee.designation])
            .await
            .map_err(|e| reject::custom(Error::QueryError(e)))?;
        designations.push(row.get(0));
    }

    employees
        .iter_mut()
        .zip(uuid.iter())
        .for_each(|(e, u)| e.password = generate_password(&e.password, u));

    let params: Vec<_> = employees
        .iter()
        .zip(uuid.iter())
        .zip(designations.iter())
        .flat_map(|((e, u), d)| e.params(u, d).to_vec().into_iter())
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

pub(super) async fn de((id, db): (Uuid, Arc<Client>)) -> Result<impl Reply, Rejection> {
    let statement = db
        .prepare_typed(
            "DELETE FROM EMPLOYEE
            WHERE ID = $1",
            &[Type::UUID],
        )
        .await
        .map_err(|e| reject::custom(Error::StatementPrepareError(e)))?;

    db.execute(&statement, &[&id])
        .await
        .map_err(|e| reject::custom(Error::ExecuteError(e)))?;

    Ok(reply::reply())
}
