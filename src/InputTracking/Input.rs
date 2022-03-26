/// Input enum handles the storage of keyboard and mouse inputs
enum Input{
    key_down{button: Keycode},
    key_up{button: Keycode},
    mouse_button_up{button: usize},
    mouse_button_down{button: usize},
    mouse_move{deltaX : i32, deltaY: i32},   
}
