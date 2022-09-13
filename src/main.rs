use std::{thread, time};
use chrono::offset::Local;

fn main() {
    //cronometer(3);
    println!("Date: {}", date());
    println!("Time: {}", time());
    println!("Hour: {}", hour());
}

fn date() -> String {
    let date: String = format!("{}", Local::now().date());
    date[0..10].to_string()
}

fn time() -> String {
    let date = format!("{}", Local::now().time());
    date[0..8].to_string()
}

fn hour() -> u8 {
    let hour = time()[0..2].to_string();
    match hour.parse::<u8>() {
        Ok(hour) => hour,
        Err(_) => 0,
    }
}

fn cronometer(stop_time: u32) {
    let one_sec = time::Duration::from_secs(1);

    for i in 0..stop_time {
        println!("{}", i);
        thread::sleep(one_sec);
    }
}