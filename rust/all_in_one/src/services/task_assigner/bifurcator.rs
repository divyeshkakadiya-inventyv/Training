use std::{sync::Arc, thread, time::Duration};

use crate::{
    services::common_st::{Language, Status, Task, TaskType},
    CALL_TASKS, CHAT_TASKS, ESCALATION, TASKS,
};

///bifurcate task on chat/call
pub fn bifurcate_task() {
    let pending_tasks = TASKS.clone();
    let chat_tasks = Arc::clone(&CHAT_TASKS);
    let call_tasks = CALL_TASKS.clone();

    //bifurcating between the chat and calls and make two different Vectors
    if pending_tasks.read().unwrap().is_empty() {
        println!("Data is in process!!");
        thread::sleep(Duration::from_secs(5));
    }
    let data = pending_tasks.write().unwrap().pop_front();
    // println!("testing pending_task{:?}", pending_tasks.read().unwrap());
    match data {
        Some(data) => {
            if data.available_at == TaskType::Call {
                call_tasks.write().unwrap().push_back(data)
            } else {
                chat_tasks.write().unwrap().push_back(data)
            }
        }
        None => println!("pending data is not available"),
    }
    // println!("call data i:: {:#?}", call_tasks.read().unwrap());
    // println!("chat data :: {:#?}", chat_tasks.read().unwrap());

    println!("Data is bifurcate!!");
}

///bifurcate on generated key by function and put in main pending task queue
pub fn bifurcate_on_escalation() {
    let ref_pending_task = Arc::clone(&TASKS);
    let ref_escalation = Arc::clone(&ESCALATION);

    if !ref_pending_task.read().unwrap().is_empty() {
        let data = if let Some(data) = ref_pending_task
            .write()
            .ok()
            .and_then(|mut e| e.pop_front())
        {
            data
        } else {
            todo!("error is facing to unwrap data");
        };

        let key = key_generator(&data, "l1");

        //writing the data on the escalation by its key
        if let Ok(escalation) = ref_escalation.read() {
            if let Some(entry) = escalation.get(&key) {
                if let Ok(mut entry_write) = entry.write() {
                    entry_write.push_back(data);
                    println!("data is bifurcate!!!");
                } else {
                    println!("Error writing");
                }
            } else {
                println!("Key not found");
            }
        } else {
            println!("Error reading");
        }
    } else {
        println!("task is not avalable for the bifurcate");
        thread::sleep(Duration::from_secs(2));
    }
}

///generate key with combination of some paramater
pub fn key_generator(data: &Task, level: &str) -> String {
    let key = format!(
        "{}_{}_{}_{}",
        if data.available_at == TaskType::Call {
            "call"
        } else {
            "chat"
        },
        data.skill,
        if data.language == Language::English {
            "English"
        } else {
            "Spanish"
        },
        level
    );
    return key;
}
