use core::time;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

mod InputTracking;

use InputTracking::input::Action;

#[macro_use] extern crate serde_derive;

use std::fs::{File, OpenOptions};
use std::io::{prelude::*, SeekFrom};

fn main() {
    let list: Arc<Mutex<Vec<Action>>> = Arc::new(Mutex::new(Vec::new()));
    let _join_handle = InputTracking::start(list.clone());

    let mut last_update: Vec<Action> = Vec::new();

    loop{
        thread::sleep(time::Duration::from_secs(5));
        let mut values = list.lock().unwrap();

        let tmp: Vec<Action> = values.iter()
            .map(|v| v.clone())
            .collect();
        if !Path::new(".json").exists() {
            last_update.clear();
            last_update = tmp.clone();
        }
        let json = serde_json::to_string(&last_update).unwrap();
        let mut file = File::create(".json").unwrap();
        file.write(json.as_bytes());

        last_update = tmp;
    }
}