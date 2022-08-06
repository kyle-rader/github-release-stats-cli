fn main() -> Result<(), reqwest::Error> {
    const USER: &str = "AzureAD";
    const REPO: &str = "microsoft-authentication-cli";

    let url = format!("https://api.github.com/repos/{USER}/{REPO}/releases");

    let client = reqwest::blocking::Client::builder()
        .user_agent("github-stats-cli")
        .build()?;

    let response = client.get(url).send()?;

    let data: serde_json::Value = response.json()?;

    if let Some(data) = data.as_array() {
        for r in data {
            println!("{}", r["name"]);
        }
    }
    Ok(())
}
