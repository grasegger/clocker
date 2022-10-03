use clap::Parser;
use cli::{Cli, Commands};
use commands::{clock_in, clock_out};

mod cli;
mod commands;
mod db;
mod model;
mod schema;

fn main() {
    let cli = Cli::parse();
    let mut connection = db::establish_connection();

    match &cli.command {
        Commands::In { when } => clock_in::execute(when, &mut connection),
        Commands::Out { when } => clock_out::execute(when, &mut connection),
        Commands::Balance => todo!(),
        Commands::Csv => todo!(),
        Commands::SetTime {
            from,
            hours_per_week,
        } => todo!(),
    }
}
