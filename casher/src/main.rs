#![feature(proc_macro_hygiene, decl_macro)]

use std::net::{SocketAddr, IpAddr};
use std::cell::{
    Cell,
    RefCell
};

use json::object;
use rocket::{get, routes};
use spin::Mutex;

const COUNT_LIMIT: u32 = 10;
static WAIT_LIST: Mutex<Vec<User>> = Mutex::new(Vec::new());
static LAST_ID: Mutex<usize> = Mutex::new(1);

#[derive(Debug)]
struct User {
    pub id: usize,
    pub time: usize
}

#[get("/register?<time>")]
fn register(time: usize) -> String {
    let mut json = object!{
        status: false
    };
    if check_count(time) < COUNT_LIMIT {
        let mut last_id = LAST_ID.lock();
        let mut wait_list = WAIT_LIST.lock();
        wait_list.push(User{id: *last_id, time});
        json = object!{
            status: true,
            id: *last_id
        };
        *last_id += 1;
    }
    return json.dump()
}

#[get("/get_count?<time>")]
fn get_count(time: usize) -> String {
    format!("{}", check_count(time))
}

#[get("/remove?<time>")]
fn remove(time: usize) -> String {
    match find_time(time) {
        Some(idx) => {
            WAIT_LIST.lock().remove(idx);
            *LAST_ID.lock() -= 1;
            return String::from("true")
        },
        None => {
            println!("invalid time");
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
    return count
}

fn find_time(time: usize) -> Option<usize> {
    let len = WAIT_LIST.lock().len();
    for i in 0..len {
        if WAIT_LIST.lock()[i].time == time {
            return Some(i)
        }
    }
    return None
}

fn main() {
    rocket::ignite().mount("/", routes![register, remove, get_count]).launch();
} 
