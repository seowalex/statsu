mod anilist;

use anilist::AniList;
use anyhow::Result;
use std::{env, process};

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        println!(
            "usage: {} username",
            args.get(0).unwrap_or(&"statsu".to_string())
        );
        process::exit(1);
    }

    let anilist = AniList::new(&args[1]);
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
