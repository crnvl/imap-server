use serde::Serialize;
use sqlx::{Pool, Postgres, Row};

#[derive(Serialize)]
pub struct IterableIP {
    pub id: i64,
    pub ip: String,
    pub latency: i64,
    pub online: bool,
}

pub async fn db_get_handle() -> Pool<Postgres> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://imap:imap@localhost:5432")
        .await
        .unwrap()
}

pub async fn get_all_rows(client: Pool<Postgres>) -> Vec<IterableIP> {
    let rows = sqlx::query("SELECT * FROM iterable_ip")
        .fetch_all(&client)
        .await
        .unwrap();

    let mut iterable_ips = Vec::new();

    for row in rows {
        let id: i64 = row.get("id");
        let ip: String = row.get("ip");
        let latency: i64 = row.get("latency");
        let online: bool = row.get("online");

        let iterable_ip = IterableIP {
            id,
            ip,
            latency,
            online,
        };

        iterable_ips.push(iterable_ip);
    }

    iterable_ips
}

pub async fn get_rows_in_range(client: Pool<Postgres>, start: i64, end: i64) -> Vec<IterableIP> {
    let rows = sqlx::query("SELECT * FROM iterable_ip WHERE id >= $1 AND id <= $2")
        .bind(start)
        .bind(end)
        .fetch_all(&client)
        .await
        .unwrap();

    let mut iterable_ips = Vec::new();

    for row in rows {
        let id: i64 = row.get("id");
        let ip: String = row.get("ip");
        let latency: i64 = row.get("latency");
        let online: bool = row.get("online");

        let iterable_ip = IterableIP {
            id,
            ip,
            latency,
            online,
        };

        iterable_ips.push(iterable_ip);
    }

    iterable_ips
}