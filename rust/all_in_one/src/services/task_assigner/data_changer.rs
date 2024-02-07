use rand::Rng;

use crate::{
    services::common_st::{Language, Status},
    EMP_DATA, SKILLS,
};

///changing the skill of the all Employees randomly
pub fn skill_changer() {
    let employees = EMP_DATA.clone();
    for emp in employees.write().unwrap().iter_mut() {
        emp.skills = random_skill_generator();
    }
}

///changing the language of the all Employees randomly
pub fn language_changer() {
    let employees = EMP_DATA.clone();
    for emp in employees.write().unwrap().iter_mut() {
        emp.language = Language::random();
    }
}

///changing the status of the all Employees randomly
pub fn status_changer() {
    let employees = EMP_DATA.clone();
    for emp in employees.write().unwrap().iter_mut() {
        emp.status = Status::random();
    }
}

///generate vector of random skills
fn random_skill_generator() -> Vec<String> {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(2, 5);
    let mut skills: Vec<String> = Vec::new();
    for _ in 1..number {
        let rand_skill = &SKILLS.read().unwrap()[rng.gen_range(0, 12)];
        skills.push(rand_skill.to_string());
    }
    // println!("{:?}" , skills);
    skills
}
