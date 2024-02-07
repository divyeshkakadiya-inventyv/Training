use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use chrono::{Timelike, Utc};
use rand::Rng;

use super::common_st::data;

fn generate_string() -> String {
    let characters: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();
    let mut name = String::new();
    for _ in 0..7 {
        let num = rng.gen_range(0, characters.len());
        name.push(characters[num] as char);
    }
    name
}

pub fn execute_task(data: Vec<data>) {
    let data = RwLock::new(data);
    let arc = Arc::new(data);
    let ref1 = arc.clone();
    let ref2 = arc.clone();
    let ref3 = arc.clone();

    let add_data = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        let mut rng = rand::thread_rng();
        let id = rng.gen_range(0, 1000);
        ref1.write()
            .unwrap()
            .push(data::new(id, generate_string(), Utc::now()));
        println!("data is added");
    });
    let length = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(3));
        println!("new vector is : {:#?}", ref2.read().unwrap());
    });
    let remove = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(10));
        ref3.write()
            .unwrap()
            .retain(|e| Utc::now().minute() - e.timestamp.minute() > 1);
    });

    add_data.join().unwrap();
    length.join().unwrap();
    remove.join().unwrap();
}
