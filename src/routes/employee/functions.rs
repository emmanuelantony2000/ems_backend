use anyhow::bail;
use tokio_postgres::types::Type;
use uuid::Uuid;

use super::{Employee, EmployeeId};
use crate::AppState;

pub(super) async fn pe(state: &AppState, employees: &[Employee]) -> anyhow::Result<Vec<Uuid>> {
    let len = employees.len();

    let mut count = 1usize;
    let mut query = String::from("INSERT INTO EMPLOYEE VALUES");

    for _ in 0..len {
        query.push_str(" (");

        for _ in 0..7 {
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
        Type::DATE,
        Type::TEXT,
        Type::INT4,
        Type::TEXT,
    ]
    .iter()
    .cycle()
    .cloned()
    .take(8 * len)
    .collect();

    let statement = state.db.prepare_typed(query.as_str(), &types).await?;

    let uuid: Vec<Uuid> = (0..len).map(|_| Uuid::new_v4()).collect();
    let params: Vec<_> = employees
        .iter()
        .zip(uuid.iter())
        .flat_map(|(e, u)| e.params(u).to_vec().into_iter())
        .collect();
    let res = state.db.execute(&statement, &params[..]).await?;

    if res != len as u64 {
        bail!("Insert unsuccessful");
    }

    Ok(uuid)
}

pub(super) async fn ge(state: &AppState, id: &Uuid) -> anyhow::Result<Employee> {
    let statement = state
        .db
        .prepare_typed(
            "SELECT
            NAME, EMAIL, PHNO, DOB, ROLE, EXPERIENCE, ADDRESS
            FROM EMPLOYEE
            WHERE ID = $1",
            &[Type::UUID],
        )
        .await?;

    let row = state.db.query_one(&statement, &[id]).await?;

    Ok(Employee {
        name: row.get(0),
        email: row.get(1),
        phno: row.get(2),
        dob: row.get(3),
        role: row.get(4),
        experience: row.get(5),
        address: row.get(6),
    })
}

pub(super) async fn ges(state: &AppState) -> anyhow::Result<Vec<EmployeeId>> {
    let statement = state
        .db
        .prepare_typed(
            "SELECT *
            FROM EMPLOYEE",
            &[],
        )
        .await?;

    let rows = state.db.query(&statement, &[]).await?;

    Ok(rows
        .iter()
        .map(|row| EmployeeId {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            phno: row.get(3),
            dob: row.get(4),
            role: row.get(5),
            experience: row.get(6),
            address: row.get(7),
        })
        .collect())
}
