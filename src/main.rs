use std::ops::Sub;

use chrono::{DateTime, Local};
use rsntp::SntpClient;

fn main() {
    const ALLOWED_UPPER_DIFFERENCE_IN_SECONDS: i8 = 60;
    const ALLOWED_LOWER_DIFFERENCE_IN_SECONDS: i8 = -60;

    let client = SntpClient::new();
    let result = client.synchronize("2.pool.ntp.org").unwrap();
    let local_time: DateTime<Local> = Local::now();
    let sntp_time: DateTime<Local> =
        DateTime::from(result.datetime().into_chrono_datetime().unwrap());
    let difference = sntp_time.signed_duration_since(local_time).num_seconds();
    println!(
        "Local time is: {} and SNTP time is: {}",
        local_time, sntp_time
    );
    println!("Time Difference is: {}", difference);
    if (difference < ALLOWED_UPPER_DIFFERENCE_IN_SECONDS.into()
        && difference > ALLOWED_LOWER_DIFFERENCE_IN_SECONDS.into())
    {
        println!("Allowed");
    } else {
        println!("Not Allowed. Panicking!!!!!!!");
        panic!();
    }
}
