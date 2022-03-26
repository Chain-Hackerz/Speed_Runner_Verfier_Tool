use device_query::Keycode;
use device_query::MouseButton;
use device_query::MousePosition;

use chrono::prelude::*;

use serde::{Deserialize, Serialize};
use serde_derive::Deserialize;
use serde_json::Result;

/// Input enum handles the storage of keyboard and mouse inputs
#[derive(Debug, Serialize, Deserialize)]
pub enum Input{
    key_down{button: u64},
    key_up{button: u64},
    mouse_button_up{button: usize},
    mouse_button_down{button: usize},
    mouse_move{deltaX : i32, deltaY: i32},   
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    input: Input,
    time_stamp: String,    
}

impl Action {
    pub fn new(input: Input) -> Action{
        Action { input: input, time_stamp: Utc::now().to_string() }
    }
    pub fn load(input: &str) -> Result<Action> {
        serde_json::from_str(input)
        //let from = start_date;
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn load_test() {
        let input = "{\"input\":{\"mouse_move\":{\"deltaX\":486,\"deltaY\":677}},\"time_stamp\":\"2022-03-26 22:58:59.495967600 UTC\"}";
    }
}