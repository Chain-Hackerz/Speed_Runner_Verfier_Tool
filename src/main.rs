use std::sync::{Arc, Mutex};

mod InputTracking;

use InputTracking::input::Action;

#[macro_use] extern crate serde_derive;

fn main() {
    let list: Arc<Mutex<Vec<Action>>> = Arc::new(Mutex::new(Vec::new()));
    let _join_handle = InputTracking::start(list.clone());

    loop{
        let mut values = list.lock().unwrap();
        values.iter()
            .map(|val| println!("{:?}", serde_json::to_string(&val)))
            .last();
        values.clear();
    }
}