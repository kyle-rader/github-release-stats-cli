use std::{fmt::Display, time::Instant};

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
            "{}\n{INDENT}{INDENT}MB: {:.2}\n{INDENT}{INDENT}downloads: {:.2}k\n",
            self.name,
            (self.size as f32) / 1_000_000f32,
            (self.download_count as f32) / 1_000f32
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
            "Release: {}\nTag: {}\n",
            self.name.clone().unwrap_or("<unnamed>".into()),
            self.tag_name.clone().unwrap_or("<untagged>".into())
        )?;

        if self.assets.len() > 0 {
            write!(f, "Assets:\n")?;
            for a in self.assets.clone() {
                write!(f, "{INDENT}{}", a)?;
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

fn main() -> Result<(), reqwest::Error> {
    let Args {
        user, repo, latest, ..
    } = Args::parse();

    let per_page = if latest { 1 } else { 5 };
    let url = format!("https://api.github.com/repos/{user}/{repo}/releases?per_page={per_page}");

    let client = reqwest::blocking::Client::builder()
        .user_agent("github-stats-cli")
        .build()?;

    let (response, response_time) = timeit! { client.get(url).send()?};

    let (data, parse_time): (Vec<Release>, u128) = timeit! {response.json()?};

    let (_, print_time) = timeit! {
        for r in data {
            println!("{r}");
        }
    };

    println!("Timings:");
    println!("{:6}: {:.2} ms", "fetch", response_time);
    println!("{:6}: {:.2} ms", "parse", parse_time);
    println!("{:6}: {:.2} ms", "print", print_time);

    Ok(())
}
