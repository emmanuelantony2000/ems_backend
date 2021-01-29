use std::sync::Arc;

use chrono::NaiveDate;
use tokio_postgres::types::{ToSql, Type};
use tokio_postgres::{Client, NoTls};
use uuid::Uuid;

use crate::auth;
use crate::error::Error;

pub async fn init() -> Result<Arc<Client>, Error> {
    let (db, connection) = tokio_postgres::connect("host=localhost user=postgres", NoTls)
        .await
        .map_err(Error::DBConnectError)?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // db.execute("DROP TABLE IF EXISTS DESIGNATION CASCADE", &[])
    //     .await
    //     .map_err(Error::ExecuteError)?;
    // db.execute("DROP TABLE IF EXISTS EMPLOYEE CASCADE", &[])
    //     .await
    //     .map_err(Error::ExecuteError)?;
    // db.execute("DROP TABLE IF EXISTS ATTENDANCE CASCADE", &[])
    //     .await
    //     .map_err(Error::ExecuteError)?;

    // db.execute(
    //     "CREATE TABLE DESIGNATION (
    //         ID UUID NOT NULL,
    //         NAME TEXT NOT NULL,
    //         PRIMARY KEY(ID),
    //         UNIQUE(NAME)
    //     )",
    //     &[],
    // )
    // .await
    // .map_err(Error::ExecuteError)?;
    // db.execute(
    //     "CREATE TABLE EMPLOYEE (
    //         ID UUID NOT NULL,
    //         NAME TEXT NOT NULL,
    //         EMAIL TEXT NOT NULL,
    //         PASSWORD TEXT NOT NULL,
    //         PHONE_NUMBER TEXT NOT NULL,
    //         DOB DATE NOT NULL,
    //         ROLE TEXT NOT NULL,
    //         DESIGNATION UUID NOT NULL REFERENCES DESIGNATION(ID),
    //         EXPERIENCE INT NOT NULL,
    //         ADDRESS TEXT NOT NULL,
    //         PRIMARY KEY(ID),
    //         UNIQUE(EMAIL)
    //     )",
    //     &[],
    // )
    // .await
    // .map_err(Error::ExecuteError)?;
    // db.execute(
    //     "CREATE TABLE ATTENDANCE (
    //         ID UUID NOT NULL,
    //         EMPLOYEE_ID UUID NOT NULL REFERENCES EMPLOYEE(ID),
    //         DATESTAMP DATE NOT NULL,
    //         TIMESTAMP TIME NOT NULL,
    //         PRIMARY KEY(ID),
    //         UNIQUE(EMPLOYEE_ID, DATESTAMP)
    //     )",
    //     &[],
    // )
    // .await
    // .map_err(Error::ExecuteError)?;
    //
    // admin(&db).await?;

    Ok(Arc::new(db))
}

async fn admin(db: &Client) -> Result<(), Error> {
    let designation_uuid = Uuid::new_v4();

    let statement = db
        .prepare_typed(
            "INSERT INTO DESIGNATION
        VALUES ($1, $2)",
            &[Type::UUID, Type::TEXT],
        )
        .await
        .map_err(Error::StatementPrepareError)?;

    db.execute(
        &statement,
        &[
            &designation_uuid as &(dyn ToSql + Sync),
            &"Admin" as &(dyn ToSql + Sync),
        ],
    )
    .await
    .map_err(Error::ExecuteError)?;

    let uuid = Uuid::new_v4();
    let password = auth::generate_password("admin", &uuid);

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
        .map_err(Error::StatementPrepareError)?;

    db.execute(
        &statement,
        &[
            &uuid as &(dyn ToSql + Sync),
            &"admin".to_string() as &(dyn ToSql + Sync),
            &"admin@admin".to_string() as &(dyn ToSql + Sync),
            &password as &(dyn ToSql + Sync),
            &"9999999999".to_string() as &(dyn ToSql + Sync),
            &NaiveDate::from_ymd(2000, 1, 1) as &(dyn ToSql + Sync),
            &"Admin".to_string() as &(dyn ToSql + Sync),
            &designation_uuid as &(dyn ToSql + Sync),
            &1 as &(dyn ToSql + Sync),
            &"admin".to_string() as &(dyn ToSql + Sync),
        ],
    )
    .await
    .map_err(Error::ExecuteError)?;

    Ok(())
}
