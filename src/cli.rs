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
        start: NaiveDate,
        #[clap(help = "Date formated as 'YYYY-MM-DD'")]
        stop: NaiveDate,
        #[clap(help = "Hours monday, either as integer or float.")]
        monday: f32,
        #[clap(help = "Hours tuesday, either as integer or float.")]
        tuesday: f32,
        #[clap(help = "Hours wednesday, either as integer or float.")]
        wednesday: f32,
        #[clap(help = "Hours thursday, either as integer or float.")]
        thursday: f32,
        #[clap(help = "Hours friday, either as integer or float.")]
        friday: f32,
        #[clap(help = "Hours saturday, either as integer or float.")]
        saturday: f32,
        #[clap(help = "Hours sunday, either as integer or float.")]
        sunday: f32,
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
