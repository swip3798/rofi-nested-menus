use std::{fs, process::exit, eprintln};

use lazy_static::initialize;

use crate::{args::ARGS, menu::execute_menu};

pub mod args;
pub mod menu;
pub mod rofi;

fn main() {
    std::panic::set_hook(Box::new(|p| {
        if let Some(s) = p.payload().downcast_ref::<String>() {
            eprintln!("{}", s);
        } else if let Some(s) = p.payload().downcast_ref::<&str>() {
            eprintln!("{}", s);
        } else {
            eprintln!("Generic error!");
        }
        exit(1);
    }));

    initialize(&ARGS);
    let json_file = fs::read_to_string(&ARGS.path).unwrap_or_else(|e| {
        panic!(
            "Could not read json file at path: {:?} because of error: {e}",
            &ARGS.path
        )
    });
    let json = json::parse(&json_file)
        .unwrap_or_else(|e| panic!("Could not parse JSON file, error: {}", e));
    execute_menu(&json);
}
