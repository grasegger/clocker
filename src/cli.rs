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
    #[clap(about = "Start a work timer - you can optionally add a date and time.")]
    In {
        #[clap(help = "Date and time formated as 'YYYY-MM-DDTHH:mm:ss'")]
        when: Option<NaiveDateTime>,
    },
    #[clap(about = "Stop a work timer - you can optionally add a date and time.")]
    Out {
        #[clap(help = "Date and time formated as 'YYYY-MM-DDTHH:mm:ss'")]
        when: Option<NaiveDateTime>,
    },
    #[clap(about = "Take a look at your current balance.")]
    Balance,
    #[clap(about = "Set your working hours peer week starting from date.")]
    SetHours {
        #[clap(help = "Date formated as 'YYYY-MM-DD'")]
        from: NaiveDate,
        #[clap(help = "Hours per week, either as integer or float.")]
        hours_per_week: f32,
    },
    #[clap(about = "Block hours for holiday, vacation and sick leave.")]
    Block {
        #[clap(help = "Date formated as 'YYYY-MM-DD'")]
        from: NaiveDate,
        #[clap(help = "Hours per week, either as integer or float.")]
        hours: f32,
        #[clap(help = "The reason the time is blocked")]
        reason: String,
    },
}
