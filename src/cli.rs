use chrono::{NaiveDate, NaiveDateTime};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    In {
        when: Option<NaiveDateTime>,
    },
    Out {
        when: Option<NaiveDateTime>,
    },
    Balance,
    Csv,
    SetHours {
        from: NaiveDate,
        hours_per_week: u8,
    },
    Block {
        from: NaiveDate,
        hours: u8,
        reason: String,
        until: Option<NaiveDate>,
    },
}
