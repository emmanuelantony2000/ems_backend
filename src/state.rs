use actix_web::rt;
use tokio_postgres::{Client, NoTls};

pub struct AppState {
    pub db: Client,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let (db, connection) =
            tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

        rt::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        db.execute("DROP TABLE EMPLOYEE", &[]).await?;
        db.execute(
            "CREATE TABLE EMPLOYEE (
            ID UUID NOT NULL,
            NAME TEXT NOT NULL,
            EMAIL TEXT NOT NULL,
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

        Ok(Self { db })
    }
}
