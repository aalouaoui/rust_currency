mod api;

use api::*;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author = "KA", version, about= "A bunch of currency related tools", long_about= None)]
#[clap(propagate_version = true)]
struct Cli {
    /// Name of the person to greet
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List all currencies
    List,
    /// Search for specific query
    Search {
        #[clap(value_parser)]
        query: String,
    },
    Convert {
        /// Source currency
        #[clap(short, long, value_parser)]
        from: String,
        /// Destination currency
        #[clap(short, long, value_parser)]
        to: String,
        /// Amount of source currency
        #[clap(value_parser, default_value_t = 1.)]
        val: f64,
    },
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Commands::List => println!("{}", list_currencies_string()?),
        Commands::Search { query } => println!("{}", search_currency(&query)?),
        Commands::Convert { from, to, val } => {
            println!(
                "{} {} = {} {}",
                val,
                from,
                convert_currency(val, &from, &to)?,
                to
            )
        }
    }
    Ok(())
}
