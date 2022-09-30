mod anilist;

use anilist::AniList;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Cli {
    /// AniList user name
    username: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let anilist = AniList::new(&cli.username);
    let franchises = anilist.get_franchises().await?;

    for franchise in &franchises {
        for entry in &franchise.entries {
            println!("{}", entry.title);
        }

        println!();
    }

    println!("{} franchises", franchises.len());

    Ok(())
}
