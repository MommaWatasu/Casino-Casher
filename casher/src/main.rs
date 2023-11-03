#![feature(proc_macro_hygiene, decl_macro)]

use chrono::prelude::*;
use chrono_tz::Asia::Tokyo;
use json::object;
use rocket::{get, post, routes};
use rocket_cors::CorsOptions;
use spin::Mutex;

const COUNT_LIMIT: u32 = 2;
// List to check waiting user
static WAIT_LIST: Mutex<Vec<String>> = Mutex::new(Vec::new());
// List to check reserved user
static RESERVE_LIST: Mutex<Vec<User>> = Mutex::new(Vec::new()); 
// Table to check count
static RESERVE_TABLE: Mutex<[u32; 14]> = Mutex::new([0; 14]);

#[derive(Clone, Copy, Debug)]
struct Time{
    hour: usize,
    minute: usize
}

impl Time {
    pub fn cmp(&self, other: Self) -> bool {
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

#[get("/reserve?<time>&<ip>")]
fn reserve(mut time: usize, ip: String) -> String {
    // status=1: default(error) code
    let json ;
    let now = Local::now().with_timezone(&Tokyo);
    let mut hour = format!("{}", now.format("%H")).parse::<usize>().unwrap();
    let mut minute = format!("{}", now.format("%M")).parse::<usize>().unwrap();
    // test time
    hour = 10; minute = 10;
    let now_time = Time{hour, minute};
    let now_time_num = (hour-9)*2-1 + if minute>=30 {1} else {0};
    if time == 0 {
        for i in 0..15 {
            if RESERVE_TABLE.lock()[i] < COUNT_LIMIT && !(Time{hour: (i+2)/2+9, minute: if (i+1)%2==1 {0} else {30}}).cmp(now_time){
                time = i+1;
                break
            }
        }
        if time == 0 {
            // status=2: time has already been full
            json = object!{
                status: 2
            };
            return json.dump();
        }
    } else {
        let specified_time = Time{hour: (time+1)/2+9, minute: if time%2==1 {0} else {30}};
        if specified_time.cmp(now_time) {
            // status=3: invalid time
            json = object!{
                status: 3
            };
            return json.dump()
        }
    }
    if time - now_time_num > 5 {
        json = object! {
            status: 4
        };
        return json.dump()
    }
    if RESERVE_TABLE.lock()[time - 1] < COUNT_LIMIT {
        let mut reserve_table = RESERVE_TABLE.lock();
        let mut reserve_list = RESERVE_LIST.lock();
        reserve_table[time - 1] += 1;
        reserve_list.push(User{ ip, time });
        // status=0: success
        json = object!{
            status: 0
        };
    } else {
        json = object!{
            status: 2
        };
    }
    return json.dump()
}

#[post("/register?<ip>")]
fn register(ip: String) -> String {
    WAIT_LIST.lock().push(ip);
    return String::from("true")
}

#[get("/check_user?<ip>")]
fn check_user(ip: String) -> String {
    let mut json = object! {
        user_type: 0
    };
    if let Some(idx) = find_reserve_ip(ip.clone()) {
        json = object! {
            user_type: 1,
            time: RESERVE_LIST.lock()[idx].time
        };
    } else if let Some(idx) = find_wait_ip(ip) {
        json = object! {
            user_type: 2,
            number: idx+1
        }
    }
    return json.dump()
}

#[post("/remove_reserve?<ip>")]
fn remove_reserve(ip: String) -> String {
    match find_reserve_ip(ip) {
        Some(idx) => {
            let mut reserve_list = RESERVE_LIST.lock();
            let time = reserve_list[idx].time;
            reserve_list.remove(idx);
            RESERVE_TABLE.lock()[time-1] -= 1;
            return String::from("true")
        },
        None => {
            println!("invalid ip");
            return String::from("false")
        }
    }
}

#[post("/remove_wait?<ip>")]
fn remove_wait(ip: String) -> String {
    match find_wait_ip(ip) {
        Some(idx) => {
            WAIT_LIST.lock().remove(idx);
            return String::from("true")
        },
        None => {
            return String::from("false")
        }
    }
}

#[post("/reception")]
fn reception() -> String {
    if WAIT_LIST.lock().pop() == None {
        return String::from("false")
    } else {
        return String::from("true")
    }
}

fn find_reserve_ip(ip: String) -> Option<usize> {
    let len = RESERVE_LIST.lock().len();
    for i in 0..len {
        if RESERVE_LIST.lock()[i].ip == ip {
            return Some(i)
        }
    }
    return None
}

fn find_wait_ip(ip: String) -> Option<usize> {
    let len = WAIT_LIST.lock().len();
    for i in 0..len {
        if WAIT_LIST.lock()[i] == ip {
            return Some(i)
        }
    }
    return None
}

fn main() {
    rocket::ignite()
        .mount("/", routes![reserve, register, check_user, remove_reserve, remove_wait, reception])
        .attach(CorsOptions::default().to_cors().expect("error"))
        .launch();
} 
