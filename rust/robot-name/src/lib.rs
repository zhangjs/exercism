use rand::prelude::*;
use std::{cell::RefCell, collections::HashSet};

thread_local!(static ROBOT_NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new()));

pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        let mut r = Robot(String::new());
        r.reset_name();
        r
    }

    pub fn name(&self) -> &str {
        println!("{}", self.0);
        &self.0
    }

    pub fn reset_name(&mut self) {
        ROBOT_NAMES.with(|c| c.borrow_mut().remove(&self.0));
        self.0 = self.uniq_name();
    }

    fn uniq_name(&mut self) -> String {
        ROBOT_NAMES.with(|c| {
            let mut used = c.borrow_mut();

            loop {
                let name = Robot::random_name();
                if !used.contains(&name) {
                    used.insert(name.clone());
                    return name;
                }
            }
        })
    }

    fn random_name() -> String {
        let mut rng = rand::thread_rng();

        format!(
            "{}{}{:03}",
            rng.gen_range(b'A'..=b'Z') as char,
            rng.gen_range(b'A'..=b'Z') as char,
            rng.gen_range(0..1000)
        )
    }
}
