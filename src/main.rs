use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct Asset {
    name: String,
    size: usize,
    download_count: usize,
}

impl Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\t\tbytes: {}\n\t\tdownloads: {}\n",
            self.name, self.size, self.download_count
        )
    }
}

#[derive(Deserialize)]
struct Release {
    name: Option<String>,
    tag_name: Option<String>,
    // created_at: String, // we can do better
    assets: Vec<Asset>,
}

impl Display for Release {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} @ {}\n",
            self.name.clone().unwrap_or("<unnamed>".into()),
            self.tag_name.clone().unwrap_or("<untagged>".into())
        )?;

        for a in self.assets.clone() {
            write!(f, "\t{}", a)?;
        }
        write!(f, "")
    }
}

fn main() -> Result<(), reqwest::Error> {
    // const USER: &str = "AzureAD";
    // const REPO: &str = "microsoft-authentication-cli";
    const USER: &str = "rust-lang";
    const REPO: &str = "rust";

    let url = format!("https://api.github.com/repos/{USER}/{REPO}/releases?per_page=100");

    let client = reqwest::blocking::Client::builder()
        .user_agent("github-stats-cli")
        .build()?;

    let response = client.get(url).send()?;

    let data: Vec<Release> = response.json()?;

    for r in data {
        println!("{r}");
    }
    Ok(())
}
