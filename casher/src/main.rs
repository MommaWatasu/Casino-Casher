#![feature(proc_macro_hygiene, decl_macro)]

use chrono::prelude::*;
use json::object;
use rocket::{get, post, routes};
use rocket_cors::CorsOptions;
use spin::{
    Mutex,
    Lazy
};

const COUNT_LIMIT: u32 = 6;
static WAIT_LIST: Mutex<Vec<User>> = Mutex::new(Vec::new());
static RESERVE_TABLE: Lazy<Mutex<Vec<u32>>> = Lazy::new(|| Mutex::new(vec![0; 14])); 

struct Time{
    hour: usize,
    minute: usize
}

impl Time {
    pub fn cmp(&self, other: Self) -> bool {
        println!("self hour: {}, other hour: {}", self.hour, other.hour);
        if self.hour < other.hour {
            return true
        } else if self.hour == other.hour && self.minute < other.minute {
            return true
        } else {
            return false
        }
    }
}

#[derive(Clone, Debug)]
struct User {
    pub ip: String,
    pub time: usize
}

#[get("/register?<time>&<ip>")]
fn register(mut time: usize, ip: String) -> String {
    let mut json = object!{
        status: false,
        err: 1
    };
    let now = Local::now();
    let hour = format!("{}", now.format("%H")).parse::<usize>().unwrap();
    println!("hour: {}", hour);
    let minute = format!("{}", now.format("%M")).parse::<usize>().unwrap();
    let now_time = Time{hour, minute};
    if time == 0 {
        for i in 0..14 {
            if RESERVE_TABLE.lock()[i] < COUNT_LIMIT {
                time = i+1;
                break
            }
        }
    }
    let specified_time = Time{hour: (time+1)/2+9, minute: if time%2==1 {0} else {30}};
    if specified_time.cmp(now_time) {
        json = object!{
            status: false,
            err: 2
        };
        return json.dump()
    }
    if check_count(time) < COUNT_LIMIT {
        let mut wait_list = WAIT_LIST.lock();
        wait_list.push(User{ip, time});
        json = object!{
            status: true
        };
    } else {
        json = object!{
            status: false,
            err: 3
        };
    }
    return json.dump()
}

#[get("/get_time?<ip>")]
fn get_time(ip: String) -> String {
    let json = object! {
        time: check_time(ip)
    };
    return json.dump()
}

#[post("/remove?<ip>")]
fn remove(ip: String) -> String {
    match find_ip(ip) {
        Some(idx) => {
            WAIT_LIST.lock().remove(idx);
            return String::from("true")
        },
        None => {
            println!("invalid ip");
            return String::from("false")
        }
    }
}

fn check_count(time: usize) -> u32 {
    let mut count = 0;
    let len = WAIT_LIST.lock().len();
    for i in 0..len {
        if WAIT_LIST.lock()[i].time == time {
            count += 1;
        }
    }
    return count;
}

fn check_time(ip: String) -> isize {
    let len = WAIT_LIST.lock().len();
    for i in 0..len {
        let user = WAIT_LIST.lock()[i].clone();
        if user.ip == ip {
            return user.time as isize
        }
    }
    return -1;
}

fn find_ip(ip: String) -> Option<usize> {
    let len = WAIT_LIST.lock().len();
    for i in 0..len {
        if WAIT_LIST.lock()[i].ip == ip {
            return Some(i)
        }
    }
    return None
}

fn main() {
    rocket::ignite()
        .mount("/", routes![register, remove, get_time])
        .attach(CorsOptions::default().to_cors().expect("error"))
        .launch();
} 
