use std::{io, time::Duration};

use clap::{CommandFactory, Parser};
use crossterm::event::{poll, read, Event};
use tiles::Tiles;

mod tiles;

#[derive(Parser, Default, Debug)]
#[command(
    author = "Lulu",
    version,
    about = "tRiPy: animate randomized tiling characters."
)]
struct Arguments {
    /// Number of milliseconds to wait between updates
    #[arg(short, long, default_value_t = 20)]
    wait: u64,

    /// The different characters to use. For example: $ trippy '\' '/'
    choices: Vec<String>,
}

fn main() -> Result<(), io::Error> {
    let args = Arguments::parse();

    if args.choices.len() < 2 {
        eprintln!(
            "There must be at least 2 choices.\n\n{}",
            Arguments::command().render_long_help()
        );
    }
    assert!(args.choices.len() > 1, "There must be at least 2 choices.");

    let choices = args
        .choices
        .iter()
        .map(|s| {
            assert!(
                s.chars().count() == 1,
                "Each choice must be 1 character long."
            );
            s.chars().next().unwrap()
        })
        .collect();

    let mut tiles = Tiles::new(choices)?;

    loop {
        tiles.randomize()?;

        if poll(Duration::from_millis(args.wait))? {
            if let Event::Key(_) = read()? {
                break;
            }
        }
    }

    Ok(())
}
