use std::sync::{Arc, Mutex};

mod InputTracking;

use InputTracking::input::Action;

fn main() {
    let list: Arc<Mutex<Vec<Action>>> = Arc::new(Mutex::new(Vec::new()));
    let tmp = InputTracking::start(list.clone());

    loop{
        let mut values = list.lock().unwrap();
        values.iter()
            .map(|val| println!("{:?}", val))
            .last();
        values.clear();
    }
}