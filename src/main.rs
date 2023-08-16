use clap::Parser;

use chrono::{NaiveDateTime, DateTime, Utc};
use chrono_tz::Tz;
use chrono_tz as ChronoTz;

use ansi_term::Style;

#[derive(Parser)]
struct Cli {
    #[clap(num_args = 1..)]
    pub timestamp: Option<Vec<i64>>,
}
fn main() {
    let args = Cli::parse();

    println!();

    if let Some(timestamps) = args.timestamp {
        for timestamp in timestamps {
            process_and_output_timestamp(timestamp);
            println!();
        }
    } else {
        let now = Utc::now();
        let timestamp = now.timestamp();
        process_and_output_timestamp(timestamp);
    }
    
}

fn process_and_output_timestamp(timestamp: i64) -> () {

    println!("{}", Style::new().bold().paint(timestamp.to_string()));

    let timestamp = if timestamp > 1_000_000_000_000 {
        timestamp / 1000
    } else {
        timestamp
    };

    let timezones_to_convert_to: Vec<Tz> = vec![
        ChronoTz::US::Eastern,
        ChronoTz::US::Central,
        ChronoTz::US::Mountain,
        ChronoTz::US::Pacific,
        ];

    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).expect("Valid datetime");
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    for timezone in timezones_to_convert_to {
        
        let date = datetime.with_timezone(&timezone).format("%a %b %e, %Y");
        let time = datetime.with_timezone(&timezone).format("%l:%M:%S %p");
        let datetime_string = format!("{} {:<12}", date, time);
        let timezone_string = format!("{:<12}", timezone.to_string());
        println!("  {}: {}", timezone_string, datetime_string);
    }

}