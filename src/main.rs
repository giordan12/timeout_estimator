// This project will help me understand how many days per month I can skip going to the office.
// My score is estimated by averaging the max 9 weeks out of the past 12 weeks. Each time I go
// to the office, I get added a 20%.
//
// Plan
// 1. I need a way to specify the system how much time I plan to spend in Dallas
// 2. The system needs to run the simulation and give me my score for this value

// From teams description:
// This is calculated by taking your BELT (Best Eight Last Twelve = last 12 weeks averaging
// the highest attended 8 weeks based on badging activity) and rounded to days of week
// in 20% increments. BELT > 5 will be considered 0% work from home

use std::{cmp::Ordering, ops::Div};

use bdays::{calendars::us::USSettlement, HolidayCalendar};
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

    // How often I won't work from the office
    #[arg(short, default_value_t = 4)]
    frequency: u8, // todo I need to define a default frequency
}

fn main() {
    let args = Args::parse();
    let days_will_miss = args.days;
    let frequency_to_work_oof = args.frequency;

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

    let mut current_day: DateTime<Local> = Local::now();

    let mut future_date = get_next_monday(current_day);
    println!("{:}", future_date);

    // We will get the number of days in the first week, based on the input parameter
    // we will get how many days we will miss. then we go to the next week and estimate
    // the number of days that we will go to the office (assume we go to the office 1 per week)
    // We will assume that in 3 weeks we will again skip the same number of days from the office
    // and repeat the process until we process 12 weeks. then we select the 9 number values
    // and get the average

    let mut week_percentages: Vec<u8> = vec![]; // This vector will hold the percentages for each week

    let max_number_of_weeks: u8 = 12;

    for week_number in 0..max_number_of_weeks {
        println!(
            "Estimating percentage of attendance for week {}",
            week_number + 1
        );

        let mut percentage_total: u8 = 0;

        // Here we need to iterate over the days of the week, we start on Monday
        // and finish on Friday

        while future_date.weekday() != Weekday::Sat {
            if cal.is_bday(future_date) {
                println!(
                    "Day {} is a business day so we need to look into it",
                    future_date
                );
                percentage_total += 20;
            } else if cal.is_holiday(future_date) {
                println!(
                    "Yupiieee, {} is a holiday so we count it for us", // TODO need to confirm if holidays are counted for me or not
                    future_date
                );
                percentage_total += 20;
            }

            future_date = future_date
                .checked_add_days(Days::new(1))
                .expect("Should be able to add days");
        }

        // Every 4 weeks we will skip the number of days that we specified in the command line argument
        if (week_number % frequency_to_work_oof) == 0 {
            // either it is the beginning of the period or 4 weeks have passed. We can substract the number of
            // days we will miss

            percentage_total -= (days_will_miss * 20) // This assumes that we will only miss less than or equal to 5 days
                                                      // TODO we need to be able to estimate the percentage that each day contributes to the total 100%
        }

        println!("Percentage for this week is {}%", percentage_total);

        week_percentages.push(percentage_total); // by this point we estimated the percentage of attendance for this week and we can add it to the list

        // By this point we have gone over all possible working days of the week
        // now we need to iterate until the next monday
        future_date = get_next_monday(future_date);
    }

    if (week_percentages.len() < 12) {
        panic!("The final list of percentages doesn't have as many values as expected");
    }

    // week_percentages holds the 12 percentages, we just need to get the 8 highest and average

    println!(
        "week percentages before sorting are: {:?}",
        week_percentages
    );
    week_percentages.sort_by(|a, b| b.cmp(a));
    println!("after sorting the percentages are: {:?}", week_percentages);

    // list is sorted by now, now we need to get the averages of the top 9
    let top_9_percentages = &week_percentages[0..8]; // I didn't know that for ranges the right and left side were inclusive
    println!("top 9 values are {:?}", top_9_percentages);

    let top_9_sum: u16 = top_9_percentages.iter().map(|x| *x as u16).sum(); // I'm not fully sure how slices and vectors work, I need to study
    let average = top_9_sum.div(top_9_percentages.len() as u16);
    print!("Final average is {:}", average);
}

fn run_simulation(days_to_work_oof: u8, frequency_to_work_oof: u8) {
    let cal = bdays::calendars::us::USSettlement;

    let mut current_day = Local::now();
    let mut future_date = get_next_monday(current_day);
    println!("Next monday from today is {:}", future_date);

    let mut week_percentages: Vec<u8> = vec![]; // This vector will hold the percentages for each week

    let max_number_of_weeks: u8 = 12;

    for week_number in 0..max_number_of_weeks {
        println!(
            "Estimating percentage of attendance for week {}",
            week_number + 1
        );

        let mut percentage_total: u8 = 0; // This var will hold the final percentage for this week

        while future_date.weekday() != Weekday::Sat {
            if cal.is_bday(future_date) {
                println!(
                    "Day {} is a business day so we need to go to work",
                    future_date
                );
                percentage_total += 20;
            } else if cal.is_holiday(future_date) {
                println!(
                    "Yupiiee, {} is a holiday so we count it for us", // TODO need to confirm if holidays count for me
                    future_date
                );
                percentage_total += 20;
            }

            future_date = future_date
                .checked_add_days(Days::new(1))
                .expect("Should be able to add days");
        }
    }
}

fn simulate_week(week_day: DateTime<Local>, calendar: &USSettlement) {
    while week_day.weekday() != Weekday::Sat {
        if calendar.is_bday(week_day) {
            println!("Day is a business day so we need to go to work");
        }
    } // TODO finish this so it doesn't fail. we need to use a DateTime<local> for it work
      // maybe we will need to to make it mutable as well
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
