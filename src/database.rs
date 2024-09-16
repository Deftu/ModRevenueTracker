use tokio_postgres::Client as DatabaseClient;

pub async fn create_tables(client: &DatabaseClient) -> Result<(), crate::error::Error> {
    // modrinth_balance = f64 (nullable)
    // curseforge_points = i64 (nullable)

    client
        .execute(
            "CREATE TABLE IF NOT EXISTS payout (
                time TIMESTAMP NOT NULL DEFAULT to_timestamp(floor((extract('epoch' from now()) / 1800 )) * 1800),
                modrinth_balance DOUBLE PRECISION,
                curseforge_points BIGINT,
                PRIMARY KEY(time)
            )",
            &[],
        )
        .await?;

    Ok(())
}

pub async fn store_balances(
    client: &tokio_postgres::Client,
    modrinth_balance: &Option<f64>,
    curseforge_points: &Option<i64>,
) -> Result<(), crate::error::Error> {
    client
        .execute(
            "INSERT INTO payout (modrinth_balance, curseforge_points) VALUES ($1, $2)
            ON CONFLICT (time) DO NOTHING",
            &[&modrinth_balance, &curseforge_points],
        )
        .await?;

    Ok(())
}
