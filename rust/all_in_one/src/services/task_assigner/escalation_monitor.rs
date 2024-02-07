use std::{
    collections::{HashMap, VecDeque},
    ops::Deref,
    sync::Arc,
};

use chrono::{Duration, Timelike, Utc};

use crate::{services::common_st::Task, ESCALATION};

use super::bifurcator::key_generator;

///check the escalation all the time and chnging the level of escalation by time in l1..l5
pub fn esc_level_monitor() {
    let mut task_channel: HashMap<String, VecDeque<Task>> = HashMap::new();
    match Arc::clone(&ESCALATION).read() {
        Ok(map) => {
            for (key, value) in map.iter() {
                if let Ok(mut vec) = value.write() {
                    if !&vec.is_empty() {
                        for i in 0..vec.len() {
                            let mut key = String::new();
                            let stay_time = Utc::now().second() - vec[i].time.second();
                            if stay_time > 30 && stay_time < 60 {
                                key.push_str(key_generator(&vec[i], "l2").as_str());
                            } else if stay_time > 60 && stay_time < 90 {
                                key.push_str(key_generator(&vec[i], "l3").as_str());
                            } else if stay_time > 90 && stay_time < 120 {
                                key.push_str(key_generator(&vec[i], "l4").as_str());
                            } else {
                                key.push_str(key_generator(&vec[i], "l5").as_str());
                            }

                            if let Some(data) = task_channel.get_mut(&key) {
                                data.push_back(vec[i].clone());
                            } else {
                                // If the deque is not available, create a new one and insert the data
                                let mut new_deque = VecDeque::new();
                                new_deque.push_back(vec[i].clone());
                                task_channel.insert(key.clone(), new_deque);
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

    //changing the escalation level from l1 to l5 according to time
    let ref_escalation = Arc::clone(&ESCALATION);
    for (key, mut value) in task_channel {
        if let Ok(map) = ref_escalation.write() {
            if let Some(vec) = map.get(&key) {
                if let Ok(mut vec) = vec.write() {
                    vec.append(&mut value);
                }
            }
        }
    }

    // println!("{:?}", Arc::clone(&ESCALATION).read().unwrap());
}
