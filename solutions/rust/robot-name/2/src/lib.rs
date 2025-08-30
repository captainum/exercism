use rand::prelude::*;
use std::cell::RefCell;

thread_local!(static USED_NAMES: RefCell<Vec<String>> = const { RefCell::new(vec![]) });

fn check_name(name: &String) -> bool {
    USED_NAMES.with(
        |value| {
            value.borrow().contains(name)
        }
    )
}

fn push_name(name: &String) {
    USED_NAMES.with(
        |value| {
            value.borrow_mut().push(name.to_string());
        }
    )
}

pub struct Robot {
    name: String,
}

fn gen_name() -> String {
    let mut name = String::new();

    loop {
        for _ in 0..2 {
            name.push(
                rand::rng().sample(rand::distr::Alphabetic).to_ascii_uppercase() as char
            );
        }
        name.push_str(&rand::rng().random_range(100..=1000).to_string());

        if !check_name(&name) {
            push_name(&name);
            break;
        }
    }

    name
}

impl Robot {
    pub fn new() -> Self {
        Self { name: gen_name() }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = gen_name();
    }
}