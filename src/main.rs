// This project will help me understand how many days per month I can skip going to the office.
// My score is estimated by averaging the max 9 weeks out of the past 12 weeks. Each time I go
// to the office, I get added a 20%.
//
// Plan
// 1. I need a way to specify the system how much time I plan to spend in Dallas
// 2. The system needs to run the simulation and give me my score for this value

use bdays::HolidayCalendar;
use chrono::{
    prelude, DateTime, Datelike, Days, Duration, Local, Months, NaiveDate, TimeZone, Weekday,
};
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

    let mut current_day = Local::now();

    // let mut next_monday: Weekday;
    // loop {
    //     next_monday = current_day.weekday();
    //     match next_monday {
    //         Weekday::Mon => break,
    //         _ => {
    //             current_day = current_day
    //                 .checked_add_days(Days::new(1))
    //                 .expect("Adding one day to date should be valid")
    //         }
    //     }
    // }

    // while (current_day.weekday() != Weekday::Mon) {
    //     current_day = current_day
    //         .checked_add_days(Days::new(1))
    //         .expect("Should be able to add days");
    // }

    // By this point the next_monday var should hold the datetime for the next monday

    let mut future_date = get_next_monday(current_day);
    println!("{:}", future_date);

    //
    // We will get the number of days in the first week, based on the input parameter
    // we will get how many days we will miss. then we go to the next week and estimate
    // the number of days that we will go to the office (assume we go to the office 1 per week)
    // We will assume that in 3 weeks we will again skip the same number of days from the office
    // and repeat the process until we process 12 weeks. then we select the 9 number values
    // and get the average
    //

    // TODO we need a list of some sort where we can hold the percentages

    let max_number_of_weeks: u8 = 12;

    for week_number in 0..max_number_of_weeks {
        println!(
            "Estimating percentage of attendance for week {}",
            week_number + 1
        );

        let mut percentage_total: u8 = 0;

        // Here we need to iterate over the days of the week, we start on Monday
        // and finish on Friday

        // TODO every 4 weeks we need to skip the amount of days that we indicated
        // in the cli arg. (We are assuming we travel every 4 weeks)

        while future_date.weekday() != Weekday::Sat {
            if cal.is_bday(future_date) {
                println!(
                    "Day {} is a business day so we need to look into it",
                    future_date
                );
                percentage_total += 20;
            } else if cal.is_holiday(future_date) {
                println!(
                    "Yupiieee, {} is a holiday so we count it for us",
                    future_date
                );
                percentage_total += 20;
            }

            future_date = future_date
                .checked_add_days(Days::new(1))
                .expect("Should be able to add days");
        }

        if (week_number % 4) == 0 {
            // either it is the beginning of the period or 4 weeks have passed. We can substract the number of
            // days we will miss

            percentage_total -= (days_will_miss * 20) // This assumes that we will only miss less than or equal to 5 days
                                                      // TODO we need to be able to estimate the percentage that each day contributes to the total 100%
        }

        println!("Percentage for this week is {}%", percentage_total);

        // By this point we have gone over all possible working days of the week
        // now we need to iterate until the next monday
        future_date = get_next_monday(future_date);
    }

    // TODO over here we should have a list with percentage of attendance
    // we sort the values in descending order and grab the first 9. We get
    // the average of these and then we get their average. That's what we print
}

// I tried other ways but I couldn't do that, this is the only thing that worked for me
fn get_next_monday<T: TimeZone>(current_date: DateTime<T>) -> DateTime<T> {
    let mut next_monday: DateTime<T> = current_date.clone();
    while next_monday.weekday() != Weekday::Mon {
        next_monday = next_monday
            .checked_add_days(Days::new(1))
            .expect("Should be able to add days");
    }
    next_monday
}
