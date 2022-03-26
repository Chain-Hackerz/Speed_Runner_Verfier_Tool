use std::sync::{Mutex, Arc};
use std::thread::{self, JoinHandle};
use device_query::{ DeviceEvents, DeviceState };
use device_query::{Keycode, MousePosition, MouseButton};

pub mod input;
mod Input;

use self::input::Action;


pub fn start(output: Arc<Mutex<Vec<Action>>>) -> JoinHandle<()> {
    thread::spawn(move || {
        let device_state = DeviceState::new();

        let output_tmp = output.clone();
        let _guard = device_state.on_key_down(move |key_code: &Keycode| {
            let button:u64 = key_code.clone() as u64;
            let mut values = output_tmp.lock().unwrap();
            values.push(Action::new(input::Input::key_down{button: button}));
        });

        
        let output_tmp = output.clone();
        let _guard = device_state.on_key_up(move |key_code: &Keycode| {
            let button:u64 = key_code.clone() as u64;
            let mut values = output_tmp.lock().unwrap();
            values.push(Action::new(input::Input::key_up{button: button}));
        });

        let output_tmp = output.clone();
        let _guard = device_state.on_mouse_down(move |mouse_button: &MouseButton| {
            let button = mouse_button.clone();
            let mut values = output_tmp.lock().unwrap();
            values.push(Action::new(input::Input::mouse_button_down{button: button}));
        });

        let output_tmp = output.clone();
        let _guard = device_state.on_mouse_up(move |mouse_button: &MouseButton| {
            let button = mouse_button.clone();
            let mut values = output_tmp.lock().unwrap();
            values.push(Action::new(input::Input::mouse_button_up {button: button}));
        });

        let output_tmp = output.clone();
        let _guard = device_state.on_mouse_move(move |mouse_button: &MousePosition| {
            let button = mouse_button.clone();
            let mut values = output_tmp.lock().unwrap();
            values.push(Action::new(input::Input::mouse_move { deltaX: button.0, deltaY: button.1 }));
        });

        loop {

        }
    })
}