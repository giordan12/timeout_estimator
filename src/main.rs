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

    run_simulation(frequency_to_work_oof, days_will_miss);
}

fn run_simulation(frequency_to_work_oof: u8, days_will_miss: u8) {
    let cal = bdays::calendars::us::USSettlement;

    let current_day = Local::now();
    let mut some_monday = get_next_monday(current_day);
    println!("Next monday from today is {:}", some_monday);

    let mut week_percentages: Vec<u8> = vec![]; // This vector will hold the percentages for each week

    let max_number_of_weeks: u8 = 12;

    for week_number in 0..max_number_of_weeks {
        println!(
            "Estimating percentage of attendance for week {}",
            week_number + 1
        );

        let mut percentage_total = get_weeks_percentage_add(&some_monday, &cal);

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
        some_monday = get_next_monday(some_monday);
    }

    println!(
        "week percentages before sorting are: {:?}",
        week_percentages
    );

    week_percentages.sort_by(|a, b| b.cmp(a));
    println!("after sorting the percentages are: {:?}", week_percentages);

    let belt = get_belt_from_percentages(week_percentages);
    print!("Final average is {:}", belt);
}

/// Returns the belt (Best Eight Last Twelve) from a list of vectors that should
/// hold the percentage of attendance for weeks.
fn get_belt_from_percentages(percentages: Vec<u8>) -> u16 {
    if (percentages.len() < 12) {
        panic!("The final list of percentages doesn't have as many values as expected");
    }
    let top_9_percentages = &percentages[0..8];
    println!("top 9 values are {:?}", top_9_percentages);
    let top_9_sum: u16 = top_9_percentages.iter().map(|x| *x as u16).sum(); // I'm not fully sure how slices and vectors work, I need to study
    top_9_sum.div(top_9_percentages.len() as u16)
}

// For this function we need to use DateTime<Local> otherwise it doesn't compile
// if we try to use a TimeZone generic parameter then we run into issues since
// the TimeZone generic does not implement the copy trait by default
// NOTE that this function won't advance week_day along, it is your responsibility
// to move it to the next date you want to start from.
fn get_weeks_percentage_add(week_day: &DateTime<Local>, calendar: &USSettlement) -> u8 {
    // TODO let's make this method clone week_day and then we can modify the clone as we want
    // here, then the caller will have to move the week_day to the beginning (Monday) of next week
    //

    let mut current_day = week_day.clone();
    let mut percentage_total: u8 = 0;
    while current_day.weekday() != Weekday::Sat {
        if calendar.is_bday(current_day) {
            println!(
                "Day {} is a business day so we need to go to work",
                current_day
            );
            percentage_total += 20; // we are assuming that each week has 5 days
        } else if calendar.is_holiday(current_day) {
            println!(
                "Yupiieee, {} is a holiday so we automatically get credit for it",
                current_day
            );
            percentage_total += 20;
        }

        current_day = current_day
            .checked_add_days(Days::new(1))
            .expect("Should be able to add days");
    }
    percentage_total
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

//todo now let's add automatic testing to make sure that this is working fine
