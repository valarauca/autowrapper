use std::fs::{metadata, read_to_string};

use clap::{App, Arg, SubCommand};
use serde_json::from_str;

use crate::actions::Actions;

pub fn build_cli() -> App<'static, 'static> {
    App::new("autowrapper")
        .author("cody laeder <codylaeder@gmail.com>")
        .version("1.0.0")
        .about("automates gui interactions")
        .set_term_width(80)
        .subcommand(SubCommand::with_name("position").about("prints cursor position"))
        .subcommand(
            SubCommand::with_name("run")
                .about("runs a configuration script")
                .arg(
                    Arg::with_name("input-file")
                        .next_line_help(true)
                        .help("the config to run")
                        .validator(validate_input_json)
                        .takes_value(true)
                        .multiple(false)
                        .required(true)
                        .index(1),
                ),
        )
}

fn validate_input_json(arg: String) -> Result<(), String> {
    match metadata(&arg) {
        Ok(m) => {
            if m.is_dir() {
                return Err(format!(
                    "failed to read input path:'{:?}' is directory, not file",
                    &arg
                ));
            }
        }
        Err(e) => {
            return Err(format!(
                "failed to stat input path:'{:?}', error:'{:?}'",
                &arg, e
            ));
        }
    };
    let input = match read_to_string(&arg) {
        Err(e) => {
            return Err(format!(
                "failed to read input path:'{:?}', error:'{:?}'",
                &arg, e
            ));
        }
        Ok(data) => data,
    };
    match from_str::<Actions>(&input) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!(
            "input path:'{:?}' contains malformed json. error:'{:?}'",
            &arg, e
        )),
    }
}
