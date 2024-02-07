use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use crate::{
    services::common_st::{Emp, Status, Task},
    CALL_TASKS, CHAT_TASKS, EMP_DATA, ESCALATION,
};

///assign task according to its avalibility of language and skill.
pub fn assign_task() {
    let chat_tasks: Arc<RwLock<VecDeque<Task>>> = CHAT_TASKS.clone();
    let emp_data: Arc<RwLock<Vec<Emp>>> = EMP_DATA.clone();
    let call_tasks: Arc<RwLock<VecDeque<Task>>> = CALL_TASKS.clone();

    println!("before ::{:#?}", chat_tasks.read().unwrap());
    println!("before :: {:#?}", call_tasks.read().unwrap());

    chat_tasks.write().unwrap().retain(|e| {
        for i in emp_data.read().unwrap().iter() {
            if i.status == Status::Online && i.skills.contains(&e.skill) && i.language == e.language
            {
                println!("Chat employee is available!!!");
                return false;
            }
        }
        true
    });

    call_tasks.write().unwrap().retain(|e| {
        for i in emp_data.read().unwrap().iter() {
            if i.status == Status::Online && i.skills.contains(&e.skill) && i.language == e.language
            {
                println!("Chat employee is available!!!");
                return false;
            }
        }
        true
    });

    println!("Employee assigned!!!----------------");
    println!("After ::{:#?}", chat_tasks.read().unwrap());
    println!("After :: {:#?}", call_tasks.read().unwrap());
}

///assigning the task on chat , skills , language (hashmap task)
pub fn assign() {
    let emp_data = Arc::clone(&EMP_DATA);
    if let Ok(mut map) = Arc::clone(&ESCALATION).write() {
        for (key, value) in map.iter_mut() {
            if let Ok(mut vec) = value.write() {
                vec.retain(|e| {
                    for i in emp_data.read().unwrap().iter() {
                        if i.status == Status::Online
                            && i.skills.contains(&e.skill)
                            && i.language == e.language
                        {
                            return false;
                        }
                    }
                    true
                })
            } else {
                println!("error is generated in iterate");
            }
        }
    } else {
        println!("map is not available or not opening");
    }
}
