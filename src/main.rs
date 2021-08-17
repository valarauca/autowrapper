#![allow(deprecated, dead_code, unused_imports)]

use std::fs::read_to_string;
use std::process::exit;

use autopilot::mouse::location;
use serde_json::from_str;

pub mod reimpl;
//pub mod move_to_pos;
pub mod actions;
use crate::actions::Actions;
pub mod cli;
pub mod key;
pub mod mouse_button;
pub mod position;
pub mod scroll;
pub mod time;
use crate::cli::build_cli;

#[windows_subsystem = "windows"]
fn main() {
    match build_cli().get_matches().subcommand() {
        ("position", _) => {
            let point = location();
            println!("x: {:?} y: {:?}", point.x, point.y);
        }
        ("run", Option::Some(args)) => {
            let input_path = args.value_of("input-file").unwrap();
            let data = read_to_string(input_path).unwrap();
            match from_str::<Actions>(&data).unwrap().do_it() {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("program halted with error:'{:?}'", e);
                    exit(1);
                }
            };
        }
        (_, _) => {
            eprintln!("unrecongized args");
            exit(1);
        }
    };
    exit(0);
}
