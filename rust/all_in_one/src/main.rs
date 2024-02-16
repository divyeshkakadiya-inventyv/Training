use std::{
    collections::{HashMap, VecDeque},
    fs,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use serde_json::{Error, Value};
use services::{common_st::Employee, rest_api::{customers::model::Customer, middlewares::get_middlewares, routes::get_routes, student::model::Student}, task_assigner::{self, assigner, request_generator, static_loader}};
use utils::db_config::put_data;
use uuid::Uuid;
use crate::{services::{
    common_st::{Emp, Task}, rest_api::{emp::model::Empl, server::start_rest_api_server}, task_assigner::data_changer
}, utils::db_config::get_data};

mod services;
mod utils;
// use services::hashmap_task::employee;
// // use services::{student,employee};
// use services::frequencies::frequency;
// use services::frequencies::string_cut;

// use crate::services::hashmap_task::student;

use lazy_static::lazy_static;

lazy_static! {
    #[derive(Debug)]
    pub static ref EMP_DATA:Arc<RwLock<Vec<Emp>>> = Arc::new(RwLock::new({
        match fs::read_to_string("./src/files/task_assigner/Master_Data.json"){
            Ok(value) => {
                let data : Result<Vec<Emp> , Error> = serde_json::from_str(&value);
                match data{
                    Ok(data) => data,
                    Err(err) => todo!("{}" , err),
                }
            }Err(err) => todo!("{}" , err),
        }
    }));

    //for pending tasks
    pub static ref TASKS:Arc<RwLock<VecDeque<Task>>> = Arc::new(RwLock::new(VecDeque::new()));

    //for bifurcating
    pub static ref CHAT_TASKS:Arc<RwLock<VecDeque<Task>>> = Arc::new(RwLock::new(VecDeque::new()));
    pub static ref CALL_TASKS:Arc<RwLock<VecDeque<Task>>> = Arc::new(RwLock::new(VecDeque::new()));

    pub static ref SKILLS: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new({
        let mut temp: Vec<String> = Vec::new();
        // Pushing data into the vector
        temp.push("Customer Service".to_string());
        temp.push("Problem-solving".to_string());
        temp.push("Product Knowledge".to_string());
        temp.push("Effective Communication".to_string());
        temp.push("Time Management".to_string());
        temp.push("Adaptability".to_string());
        temp.push("Team Collaboration".to_string());
        temp.push("Feedback Analysis".to_string());
        temp.push("Proactive Engagement".to_string());
        temp.push("Technical Proficiency".to_string());
        temp.push("Cultural Sensitivity".to_string());
        temp.push("Documentation".to_string());
        temp
    }));

    #[derive(Debug)]
    pub static ref ESCALATION : Arc<RwLock<HashMap<String , Arc<RwLock<VecDeque<Task>>>>>> = Arc::new(RwLock::new(HashMap::new()));

    pub static ref STUDENT: Arc<RwLock<HashMap<String , Student>>> = Arc::new(RwLock::new({
        match fs::read_to_string("./src/files/StudentData.json"){
            Ok(value) => {
                let data : Result<Vec<Student> , Error> = serde_json::from_str(&value);
                match data{
                    Ok(vec) => {
                        let mut map:HashMap<String , Student> = HashMap::new();
                        for i in vec.into_iter(){
                            let id = Uuid::new_v4().to_string();
                            map.insert(id , i);
                        }
                        map
                    },
                    Err(err) => todo!("{}" , err),
                }
            }Err(err) => todo!("{}" , err),
        }
    }));
    pub static ref EMP : Arc<RwLock<HashMap<String , Empl>>> = Arc::new(RwLock::new({
        match fs::read_to_string("./src/files/Employee.json"){
            Ok(value) => {
                let data : Result<Vec<Empl> , Error> = serde_json::from_str(&value);
                match data{
                    Ok(vec) => {
                        let mut map:HashMap<String , Empl> = HashMap::new();
                        for i in vec.into_iter(){
                            let id = Uuid::new_v4().to_string();
                            map.insert(id , i);
                        }
                        map
                    },
                    Err(err) => todo!("{}" , err),
                }
            }Err(err) => todo!("{}" , err),
        }
    }));
    pub static ref CUSTOMER : Arc<RwLock<Vec<Customer>>> = Arc::new(RwLock::new({
        match fs::read_to_string("./src/files/task_assigner/Master_Data.json"){
            Ok(value) => {
                let data : Result<Vec<Customer> , Error> = serde_json::from_str(&value);
                match data{
                    Ok(data) => data,
                    Err(err) => todo!("{}" , err),
                }
            }Err(err) => todo!("{}" , err),
        }
    }));
    
}

// fn main() {
    // Uncomment the following lines to use the modules
    // println!("employee data is {}", employee::insert_data());
    // println!("student data is {}", student::insert_data());

    //taking data in string format
    // let str1 : Vec<char> = "Ipsum Lorem".to_lowercase().chars().collect();
    // let str2 : Vec<char> = "Greedy Algorithm".to_lowercase().chars().collect();

    // //calculate the frequencies of each
    // let frequesncies = frequency::merge_frequency(&mut frequency::get_frequency(str1) ,&mut  frequency::get_frequency(str2));

    // //sorting the frequncies vector
    // let mut common_fr = frequency::sort_freq(frequesncies.0);

    // //getting the string cut
    // let str = string_cut::cut();

    // //final output where the string is placed
    // println!("final string : {:?}" , frequency::fit_into_string(str , &mut common_fr));

    // println!("{:?}" , employee::insert_data_using_hashmap());
    // println!("{:?}" , student::insert_data_using_hashmap());

    // make_table::make();

    // let mut str = "Description(Type)";
    // println!("{}" , calculate_height(&mut str.to_string() , &18 , 48));
    // println!()

    // thread task
    // execute_task(Vec::new());

    // task_assigner task ----------------------------------------------------------------------

    // load the data to in lazy static
    // static_loader::escalation_generator(); //load static data in escalation hashmap

    // // println!("Data is loaded succesfully!!");

    // let data_adder = thread::spawn(move || loop {
    //     thread::sleep(Duration::from_secs(1));
    //     request_generator::random_task_generator();
    //     println!("data is added");
    // });

    // // bifurcate the request on chat/call
    // // let bifurcater = thread::spawn(move || loop {
    // //     thread::sleep(Duration::from_secs(3));
    // //     task_assigner::bifurcator::bifurcate_task();
    // // });

    // //bifercate on all of the data like call/chat , skills , lan , levels
    // let bifurcater_esc = thread::spawn(move || loop {
    //     thread::sleep(Duration::from_secs(2));
    //     task_assigner::bifurcator::bifurcate_on_escalation();
    // });

    // let escalation_monitor = thread::spawn(move || loop {
    //     thread::sleep(Duration::from_secs(4));
    //     task_assigner::escalation_monitor::esc_level_monitor();
    //     println!("escalation level is changed!!");
    // });

    // // assign the task if the emp is available
    // let assigner = thread::spawn(move || loop {
    //     thread::sleep(Duration::from_secs(3));
    //     assigner::assign();
    //     println!("Task is assigned!!!");
    // });

    // let skill_changer = thread::spawn(move || loop {
    //     thread::sleep(Duration::from_secs(10));
    //     data_changer::skill_changer();
    //     println!("skill is Shuffled!!");
    // });
    // let language_changer = thread::spawn(move || loop {
    //     thread::sleep(Duration::from_secs(10));
    //     data_changer::language_changer();
    //     println!("language is Shuffled!!");
    // });
    // let status_changer = thread::spawn(move || loop {
    //     thread::sleep(Duration::from_secs(10));
    //     data_changer::status_changer();
    //     println!("status is Shuffle!!");
    // });

    // data_adder.join().unwrap();
    // // bifurcater.join().unwrap();
    // bifurcater_esc.join().unwrap();
    // escalation_monitor.join().unwrap();
    // assigner.join().unwrap();
    // skill_changer.join().unwrap();
    // language_changer.join().unwrap();
    // status_changer.join().unwrap();
    
// }

#[tokio::main]
async fn main(){
    //for rest_api only 
    //starting the server
    start_rest_api_server().await;
}