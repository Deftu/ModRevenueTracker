use dotenv::dotenv;
use tokio_postgres::NoTls;

pub mod database;
pub mod error;
pub mod platform;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    dotenv().ok();

    let database_name = std::env::var("DATABASE_NAME")?;
    let database_user = std::env::var("DATABASE_USER")?;
    let database_password = std::env::var("DATABASE_PASSWORD")?;
    let database_host = std::env::var("DATABASE_HOST")?;
    let database_port = std::env::var("DATABASE_PORT")?;

    let (database_client, database_connection) = tokio_postgres::connect(
        &format!(
            "host={} port={} user={} password={} dbname={}",
            database_host, database_port, database_user, database_password, database_name
        ),
        NoTls,
    )
    .await?;

    // Open the connection itself in a separate task
    tokio::spawn(async move {
        if let Err(e) = database_connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    database::create_tables(&database_client).await?;

    let reqwest_client = reqwest::Client::new();

    let modrinth_token = std::env::var("MODRINTH_TOKEN")?;
    let modrinth_balance = platform::get_modrinth_balance(&reqwest_client, &modrinth_token).await?;

    let curseforge_cookie = std::env::var("CURSEFORGE_COOKIE")?;
    let curseforge_points =
        platform::get_curseforge_balance(&reqwest_client, &curseforge_cookie).await?;

    database::store_balances(&database_client, modrinth_balance, curseforge_points).await?;

    println!("Balances stored successfully:");
    println!(
        "- Modrinth: ${}",
        platform::modrinth_balance_as_usd(modrinth_balance.unwrap_or(0.0))
    );
    println!(
        "- CurseForge: {} (${})",
        curseforge_points.unwrap_or(0),
        platform::curseforge_points_to_usd(curseforge_points.unwrap_or(0))
    );

    Ok(())
}
