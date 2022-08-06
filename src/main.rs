use std::{fmt::Display, time::Instant};

use clap::Parser;
use serde::Deserialize;

#[derive(Debug, Parser)]
struct Args {
    /// the user
    user: String,
    /// the repo
    repo: String,
    /// Only the latest release
    #[clap(short, long)]
    latest: bool,
}

const INDENT: &str = "  ";

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
            "{}\n{INDENT}{INDENT}bytes: {}\n{INDENT}{INDENT}downloads: {}\n",
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
            write!(f, "{INDENT}{}", a)?;
        }
        write!(f, "")
    }
}

fn main() -> Result<(), reqwest::Error> {
    let Args { user, repo, latest } = Args::parse();

    let url = format!("https://api.github.com/repos/{user}/{repo}/releases?per_page=5");

    let client = reqwest::blocking::Client::builder()
        .user_agent("github-stats-cli")
        .build()?;

    let now = Instant::now();
    let response = client.get(url).send()?;
    println!("web request took {} ms.", now.elapsed().as_millis());

    let now = Instant::now();
    let data: Vec<Release> = response.json()?;
    println!("parsing took {} ms.", now.elapsed().as_millis());

    let now = Instant::now();
    for r in data.iter().take(if latest { 1 } else { data.len() }) {
        println!("{r}");
    }
    println!("printing took {} ms.", now.elapsed().as_millis());
    Ok(())
}
