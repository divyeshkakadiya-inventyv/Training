use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use crate::{services::common_st::Skills, ESCALATION, SKILLS};

pub fn escalation_generator() {
    let ref_escalation = Arc::clone(&ESCALATION);
    let ref_skills = Arc::clone(&SKILLS);
    let language = ["English", "Spanish"];
    let levels = ["l1", "l2", "l3", "l4", "l5"];
    let avalaible = ["call", "chat"];

    for av in avalaible {
        for skill in ref_skills.read().unwrap().iter() {
            for lan in language {
                for lev in levels {
                    let s = format!("{}_{}_{}_{}", av, skill, lan, lev);
                    ref_escalation
                        .write()
                        .unwrap()
                        .insert(s, Arc::new(RwLock::new(VecDeque::new())));
                }
            }
        }
    }

    // println!("{:#?}", ref_escalation.read().unwrap());
}
