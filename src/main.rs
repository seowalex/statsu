mod anilist;

use anilist::AniList;
use anyhow::Result;
use std::{env, process};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: {} username", args[0]);
        process::exit(1);
    }

    let anilist = AniList::new(&args[1]);
    let franchises = anilist.get_franchises().await?;

    for franchise in franchises {
        println!("\x1b[1m{}\x1b[0m", franchise.title);

        for entry in franchise.entries {
            println!("{}", entry.title);
        }

        println!();
    }

    Ok(())
}
