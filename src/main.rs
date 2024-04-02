// This project will help me understand how many days per month I can skip going to the office.
// My score is estimated by averaging the max 9 weeks out of the past 12 weeks. Each time I go
// to the office, I get added a 20%.
//
// Plan
// 1. I need a way to specify the system how much time I plan to spend in Dallas
// 2. The system needs to run the simulation and give me my score for this value

use bdays::HolidayCalendar;
use chrono::{prelude, Datelike, Days, Local, Months, NaiveDate, Weekday};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of days you will not go to the office
    #[arg(short)]
    days: u8,
}

fn main() {
    // How to pass how much I plan to stay away? I think the unit needs to be in days
    // Also I should keep in mind that the week has 5 working days.

    let args = Args::parse();
    let days_will_miss = args.days;

    // We will assume that all days to miss will begin counting from Monday
    // TODO handle specifying different days to start the missing period
    // e.g. If I say that I will miss 5 days, then it means that I will miss from Monday trough Friday.
    // Also, we assume that the days value is the amount of days we will miss per month
    // TODO handle how to say I will miss x days every y weeks
    // Also we will assume that beginning date for this will start the closest Monday after today
    // (calculated at execution time)
    // TODO Should be able to say when I will start missing days

    // We need to be able to get the number of days over the next 12 weeks.
    // If I say I will miss 5 days out of the next month, then I need to get
    // the number of working days over the next 4 weeks and substract the number
    // of days that I will miss. Then I repeat this 3 times to cover the next 12 weeks.
    // Finally, for every week, I estimate the percentage and provide the final average
    // of the 9 maximum percentages.
    //

    // I need to find a way to get the number of days in the next 12 weeks. Based on that,
    // I can get how many days I would miss and then group the rest in groups of 5.
    //

    let cal = bdays::calendars::us::USSettlement;
    let work_checks_period = chrono::TimeDelta::weeks(9);

    let mut current_time = Local::now();

    let mut next_monday: Weekday;
    loop {
        next_monday = current_time.weekday();
        match next_monday {
            Weekday::Mon => break,
            _ => {
                current_time = current_time
                    .checked_add_days(Days::new(1))
                    .expect("Adding one day to date should be valid")
            }
        }
    }

    // By this point the next_monday var should hold the datetime for the next monday
    println!("{:}", current_time)
}
