use std::{fmt::Display, fs, time::Instant};

use clap::{clap_derive::ArgEnum, Parser};
use serde::Deserialize;

#[derive(Debug, ArgEnum, Clone)]
enum OutputMode {
    Text,
    Json,
}

#[derive(Debug, Parser)]
struct Args {
    /// the user
    user: String,
    /// the repo
    repo: String,
    /// Only the latest release
    #[clap(short, long)]
    latest: bool,
    /// output mode
    #[clap(arg_enum)]
    output: Option<OutputMode>,
}

#[derive(Deserialize, Clone)]
struct Asset {
    name: String,
    size: usize,
    download_count: usize,
}

impl Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(
            f,
            "{:2}{:2} {:.2}MB",
            "",
            "📦",
            (self.size as f32) / 1_000_000f32
        )?;
        writeln!(
            f,
            "{:2}{:3} {:.2}k",
            "",
            "↘️",
            (self.download_count as f32) / 1_000f32
        )
    }
}

#[derive(Deserialize)]
struct Release {
    name: Option<String>,
    tag_name: Option<String>,
    created_at: String, // we can do better
    assets: Vec<Asset>,
}

impl Display for Release {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Release {}\nTag     {}\nCreated {}\n",
            self.name.clone().unwrap_or_else(|| "<unnamed>".into()),
            self.tag_name.clone().unwrap_or_else(|| "<untagged>".into()),
            self.created_at,
        )?;

        if !self.assets.is_empty() {
            writeln!(f, "Assets")?;
            for a in self.assets.clone() {
                writeln!(f, "• {}", a)?;
            }
        }
        write!(f, "")
    }
}

macro_rules! timeit {
    ($e:expr) => {{
        let now = Instant::now();
        let val = $e;
        (val, now.elapsed().as_millis())
    }};
}

fn main() -> anyhow::Result<()> {
    let Args {
        user, repo, latest, ..
    } = Args::parse();

    let per_page = if latest { 1 } else { 5 };
    let id = format!("{user}/{repo}");
    let cache = format!(".ghrs.{}.cache", id.replace('/', "."));
    let cache = std::path::Path::new(&cache);
    let url = format!("https://api.github.com/repos/{id}/releases?per_page={per_page}");

    let client = reqwest::blocking::Client::builder()
        .user_agent("github-stats-cli")
        .build()?;

    let (response, response_time) = timeit! {
        if cache.exists() {
            fs::read_to_string(cache)?
        }
        else {
            let content = client.get(url.clone()).send()?.text()?;
            fs::write(cache, &content)?;
            content
        }
    };

    let (data, parse_time): (Vec<Release>, u128) = timeit! { serde_json::from_str(&response)? };

    for r in data {
        println!("{r}");
    }

    println!("From: {url}");
    println!("Timings:");
    println!("{:6}: {:.2} ms", "fetch", response_time);
    println!("{:6}: {:.2} ms", "parse", parse_time);

    Ok(())
}
