use dotenv::dotenv;
use tokio_postgres::NoTls;

pub mod database;
pub mod error;
pub mod platform;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    dotenv()?;

    let modrinth_token = std::env::var("MODRINTH_TOKEN")?;
    let curseforge_cookie = std::env::var("CURSEFORGE_COOKIE")?;

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

    let modrinth_balance = platform::get_modrinth_balance(&reqwest_client, &modrinth_token).await;
    let curseforge_points =
        platform::get_curseforge_balance(&reqwest_client, &curseforge_cookie).await;

    let optional_modrinth_balance = modrinth_balance.ok().map(platform::modrinth_balance_as_usd); // Convert to USD - Thankfully Modrinth provides an actual currency
    let optional_curseforge_points = curseforge_points
        .ok()
        .map(platform::curseforge_points_to_usd); // CurseForge points are unfortunately not a real currency and are thus harder to understand. Though, they can be converted to USD rather easily, the `platform::curseforge_points_to_usd` function gives you a good formula to start with.

    database::store_balances(
        &database_client,
        &optional_modrinth_balance,
        &optional_curseforge_points,
    )
    .await?;

    if optional_modrinth_balance.is_some() || optional_curseforge_points.is_some() {
        println!("Balances stored successfully:");
    } else {
        println!("No balances stored.");
    }

    if let Some(modrinth_balance) = optional_modrinth_balance {
        println!("Modrinth: {}", modrinth_balance);
    } else {
        println!("Modrinth: N/A");
    }

    if let Some(curseforge_points) = optional_curseforge_points {
        println!("CurseForge: {}", curseforge_points);
    } else {
        println!("CurseForge: N/A");
    }

    Ok(())
}
