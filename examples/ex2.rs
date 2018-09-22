extern crate chrono;
use chrono::prelude::*;

use chrono::DateTime;

fn main() {
    println!("Example2");
    let mut a: Vec<i32> = Vec::new();
    for n in 1..=5 {
        println!("n={}", n);
        a.push(n);
    }
    println!("a3={}",a[3]);

//    let datetime = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
    let timestamp = 1530640012;//datetime.timestamp();
    let naive_datetime = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime_again: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);

    println!("{}", datetime_again);
}
