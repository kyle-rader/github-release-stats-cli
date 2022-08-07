use std::{
    env,
    fmt::Display,
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};

use clap::{clap_derive::ArgEnum, Parser};
use serde::Deserialize;

#[derive(Debug, ArgEnum, Clone)]
enum OutputMode {
    Text,
    Json,
}

#[derive(Debug, Parser)]
#[clap(version)]
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
    /// Pull from the cache file
    #[clap(short, long)]
    cached: bool,
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
        (val, now.elapsed())
    }};
}

fn main() -> anyhow::Result<()> {
    let Args {
        user,
        repo,
        latest,
        cached,
        ..
    } = Args::parse();

    let per_page = if latest { 1 } else { 5 };
    let id = format!("{user}/{repo}");

    let cache_file = format!("{}.cache", id.replace('/', "."));
    let cache_file: PathBuf = env::temp_dir().join(&cache_file);

    let url = format!("https://api.github.com/repos/{id}/releases?per_page={per_page}");

    let (response, response_time) = timeit! {
        if cached && cache_file.exists() {
            fs::read_to_string(cache_file)?
        }
        else {
            let client = reqwest::blocking::Client::builder()
                .user_agent("ghrs")
                .build()?;
            let content = client.get(url.clone()).send()?.text()?;
            fs::write(cache_file, &content)?;
            content
        }
    };

    let (data, parse_time): (Vec<Release>, Duration) = timeit! { serde_json::from_str(&response)? };

    for r in data {
        println!("{r}");
    }

    println!("From: {url}");
    println!("Timings:");
    println!("{:6}: {:.2} ms", "fetch", response_time.as_millis());
    println!("{:6}: {:.2} μs", "parse", parse_time.as_micros());

    Ok(())
}
