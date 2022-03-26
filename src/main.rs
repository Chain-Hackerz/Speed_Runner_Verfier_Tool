use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
use std::{thread, time};

fn main() {
    let device_state = DeviceState::new();
    loop{
        let mouse: MouseState = device_state.get_mouse();
        println!("Current Mouse Coordinates: {:?}", mouse.coords);
        let keys: Vec<Keycode> = device_state.get_keys();
        println!("Is A pressed? {:?}", keys);

        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
    }
}