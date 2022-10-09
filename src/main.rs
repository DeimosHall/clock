use std::{thread, time};
use chrono::offset::Local;

fn main() {
    //cronometer(3);
    println!("Date: {}", date());
    println!("Time: {}", time());
    println!("Hour: {}", hour());
    println!("Minute: {}", minute());
    println!("Second: {}", second());

    stop_watch(5);
    cronometer();
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

fn minute() -> u8 {
    let minute = time()[3..5].to_string();
    match minute.parse::<u8>() {
        Ok(minute) => minute,
        Err(_) => 0,
    }
}

fn second() -> u8 {
    let second = time()[6..8].to_string();
    match second.parse::<u8>() {
        Ok(second) => second,
        Err(_) => 0,
    }
}

fn stop_watch(stop_time: u32) {
    let one_sec = time::Duration::from_secs(1);

    for i in 0..stop_time {
        println!("{}", i);
        thread::sleep(one_sec);
    }
}

fn cronometer() {
    let one_sec = time::Duration::from_secs(1);
    let mut i = 0;

    loop {
        println!("{}", i);
        thread::sleep(one_sec);
        i += 1;
    }
}