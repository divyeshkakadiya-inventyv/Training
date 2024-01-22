mod services;
use services::{student,employee};
use services::frequencies::frequency;
use services::frequencies::string_cut;

fn main() {

    /// Uncomment the following lines to use the modules 
    // println!("employee data is {}", employee_insert_data());
    // println!("student data is {}", student_insert_data());
    
    ///taking data in string format
    let str1 : Vec<char> = "Ipsum Lorem".to_lowercase().chars().collect();
    let str2 : Vec<char> = "Greedy Algorithm".to_lowercase().chars().collect();
    
    ///calculate the frequencies of each
    let frequesncies = frequency::merge_frequency(&mut frequency::get_frequency(str1) ,&mut  frequency::get_frequency(str2));
    
    ///sorting the frequncies vector
    let mut common_fr = frequency::sort_freq(frequesncies.0);
    
    ///getting the string cut
    let str = string_cut::cut();

    ///final output where the string is placed
    println!("final string : {:?}" , frequency::fit_into_string(str , &mut common_fr));
}
