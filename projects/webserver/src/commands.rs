mod shutdown;
use std::{collections::HashMap, hash::RandomState};

use shutdown::SHUTDOWN_CMDS;
mod status;
use status::STATUS_CMDS;
mod helper;
use helper::HELPER_CMDS;

#[derive(Debug)]
pub enum Command {
    Shutdown,
    Status,
    Help,
}

pub fn command_mapper() -> HashMap<&'static str, Command, RandomState> {
    let mut map = HashMap::new();

    for shutdown_cmd in SHUTDOWN_CMDS.iter() {
        map.insert(*shutdown_cmd, Command::Shutdown);
    }

    for status_cmd in STATUS_CMDS.iter() {
        map.insert(*status_cmd, Command::Status);
    }

    for helper_cmd in HELPER_CMDS.iter() {
        map.insert(*helper_cmd, Command::Help);
    }

    // println!("HashMap commands have been built:");
    // for (key, value) in map.iter() {
    //     println!("Key: '{}' Command: {:?}", *key, *value);
    // }

    map
}
