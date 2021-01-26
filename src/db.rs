use std::sync::Arc;

use data_encoding::HEXUPPER;
use ring::digest;
use tokio_postgres::{Client, NoTls};
use uuid::Uuid;

pub async fn init() -> anyhow::Result<Arc<Client>> {
    let (db, connection) = tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let _ = db.execute("DROP TABLE EMPLOYEE", &[]).await;
    db.execute(
        "CREATE TABLE EMPLOYEE (
            ID UUID NOT NULL,
            NAME TEXT NOT NULL,
            EMAIL TEXT NOT NULL,
            PASSWORD TEXT NOT NULL,
            PHNO TEXT NOT NULL,
            DOB DATE NOT NULL,
            ROLE TEXT NOT NULL,
            EXPERIENCE INT NOT NULL,
            ADDRESS TEXT NOT NULL,
            PRIMARY KEY(ID),
            UNIQUE(EMAIL)
        )",
        &[],
    )
    .await?;

    admin(&db).await?;

    Ok(Arc::new(db))
}

async fn admin(db: &Client) -> anyhow::Result<()> {
    let uuid = Uuid::new_v4();
    let password = HEXUPPER
        .encode(digest::digest(&digest::SHA256, format!("{}admin", uuid).as_bytes()).as_ref());

    let query = format!("INSERT INTO EMPLOYEE VALUES (\'{}\', \'admin\', \'admin@admin\', \'{}\', \'9999999999\', \'2000-01-01\', \'admin\', \'1\', \'admin\')", uuid, password);
    let statement = db.prepare(query.as_str()).await?;

    db.execute(&statement, &[]).await?;

    Ok(())
}
