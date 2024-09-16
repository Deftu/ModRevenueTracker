use reqwest::Client as ReqwestClient;

pub async fn get_modrinth_balance(
    client: &ReqwestClient,
    token: &str,
) -> Result<Option<f64>, crate::error::Error> {
    let response = client
        .get("https://api.modrinth.com/v3/payout/balance")
        .header("Authorization", token)
        .send()
        .await?;

    let response = response.json::<serde_json::Value>().await?;

    let available = response["available"].as_str();
    let pending = response["pending"].as_str();

    if available.is_none() && pending.is_none() {
        return Ok(None);
    }

    let available = available
        .map(|s| s.parse::<f64>())
        .transpose()
        .map_err(|_| crate::error::Error::BalanceUnavailable {
            platform: "Modrinth".to_string(),
            error: crate::error::BalanceError::ParseError("available"),
            json: response.clone(),
        })?;

    let pending = pending.map(|s| s.parse::<f64>()).transpose().map_err(|_| {
        crate::error::Error::BalanceUnavailable {
            platform: "Modrinth".to_string(),
            error: crate::error::BalanceError::ParseError("pending"),
            json: response.clone(),
        }
    })?;

    let available = available.unwrap_or(0.0);
    let pending = pending.unwrap_or(0.0);

    Ok(Some(available + pending))
}

pub async fn get_curseforge_balance(
    client: &ReqwestClient,
    cookie: &str,
) -> Result<Option<i64>, crate::error::Error> {
    let response = client
        .get("https://authors.curseforge.com/_api/reward-store/user-points")
        .header("Cookie", format!("cf_auth={}", cookie))
        .send()
        .await?;

    let response = response.json::<serde_json::Value>().await?;

    let user_points = response["userPoints"].as_i64();

    Ok(user_points)
}

// Round down to two decimal places
pub fn modrinth_balance_as_usd(balance: f64) -> f64 {
    (balance * 100.0).floor() / 100.0
}

// Convert points to USD then round down to two decimal places
pub fn curseforge_points_to_usd(points: i64) -> f64 {
    let usd = (points as f64 / 100.0) * 5.0;
    (usd * 100.0).floor() / 100.0
}
