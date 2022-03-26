use device_query::Keycode;
use device_query::MouseButton;
use device_query::MousePosition;

use chrono::prelude::*;

/// Input enum handles the storage of keyboard and mouse inputs
#[derive(Debug)]
pub enum Input{
    key_down{button: Keycode},
    key_up{button: Keycode},
    mouse_button_up{button: MouseButton},
    mouse_button_down{button: MouseButton},
    mouse_move{deltaX : i32, deltaY: i32},   
}

#[derive(Debug)]
pub struct Action {
    input: Input,
    time_stamp: DateTime<Utc>,
}

impl Action {
    pub fn new(input: Input) -> Action{
        Action { input: input, time_stamp: Utc::now() }
    }
}