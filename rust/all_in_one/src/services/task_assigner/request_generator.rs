use std::sync::Arc;

use chrono::Utc;
use rand::Rng;

use crate::{
    services::common_st::{Language, Task, TaskType},
    SKILLS, TASKS,
};

///genrate random task to assign the employees
pub fn random_task_generator() {
    let ref1 = Arc::clone(&TASKS);
    let mut rng = rand::thread_rng();
    let rand_skill = &SKILLS.read().unwrap()[rng.gen_range(0, 12)];
    ref1.write().unwrap().push_back(Task::new(
        rand_skill.to_string(),
        TaskType::random(),
        Language::random(),
        Utc::now(),
    ));
    // println!("Pending - Tasks:: {:#?}", ref1.read().unwrap());
}
