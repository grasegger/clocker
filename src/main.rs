use clap::Parser;
use cli::{Cli, Commands};
use commands::{balance, block, clock_in, clock_out, set_hours};

mod cli;
mod commands;
mod db;
mod model;
mod schema;

fn main() {
    let cli = Cli::parse();
    let mut connection = db::establish_connection(None);

    match &cli.command {
        Commands::In { when } => clock_in::execute(when, &mut connection),
        Commands::Out { when } => clock_out::execute(when, &mut connection),
        Commands::Balance => balance::execute(&mut connection),
        Commands::SetHours {
            start,
            stop,
            monday,
            tuesday,
            wednesday,
            thursday,
            friday,
            saturday,
            sunday,
        } => set_hours::execute(
            start,
            stop,
            *monday,
            *tuesday,
            *wednesday,
            *thursday,
            *friday,
            *saturday,
            *sunday,
            &mut connection,
        ),
        Commands::Block {
            from,
            hours,
            reason,
        } => block::execute(from, *hours, reason, &mut connection),
    }
}
