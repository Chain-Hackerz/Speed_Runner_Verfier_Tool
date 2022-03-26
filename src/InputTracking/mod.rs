use std::sync::{mpsc, Mutex};

extern crate queues;

use queues::*;

use device_query::{DeviceEvents, DeviceState, CallbackGuard};
use device_query::Keycode;
use device_query::MouseButton;
use device_query::MousePosition;

use self::Input::Action;

pub mod Input;

pub struct Listeners{
    _key_down: CallbackGuard<fn(&Keycode)>,
    _key_up: CallbackGuard<fn(&Keycode)>,
    _mouse_move: CallbackGuard<fn(&MousePosition)>,
    _mouse_up: CallbackGuard<fn(&usize)>,
    _mouse_down: CallbackGuard<fn(&usize)>
}

pub fn start(output: Mutex<Vec<Action>>) -> Listeners {

    let device_state = DeviceState::new();

    let output2 = output.clone();

    let tmp : Listeners = Listeners {
        _key_down: device_state.on_key_down(|key_code: &Keycode| {
            println!("key down:{}", key_code);
            let _guard = &output.lock().unwrap();
            output.push(
                Action::new(
                    Input::key_down{
                        button: key_code.clone()
                    }
                )
            );
        }),
        _key_up: device_state.on_key_up(move |key_code: &Keycode| {
            println!("key up:{}", key_code);
            say_hello_world();
        }),
        _mouse_move: device_state.on_mouse_move(move |mouse_pos: &MousePosition| {
            println!("mouse move:{:?}", mouse_pos);
            say_hello_world();
        }),
        _mouse_up: device_state.on_mouse_up(move |mouse_button: &MouseButton| {
            println!("mouse move:{:?}", mouse_button);
            say_hello_world();
        }),
        _mouse_down: device_state.on_mouse_down(move |mouse_button: &MouseButton| {
            println!("mouse move:{:?}", mouse_button);
            say_hello_world();
        }),
    };

    tmp
}