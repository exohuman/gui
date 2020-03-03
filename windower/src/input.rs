pub enum InputState {
    Pressed { magnitude: i32 },
    Location { x: i32, y: i32 },
    Axis { x: i32, y: i32 },
}

pub enum InputEvent {
    Keyboard { id: u32, code: u32, state: InputState },
    Gamepad { id: u32, code: u32, state: InputState },
    Mouse { id: u32, code: u32, state: InputState },
    Other { id: u32, code: u32, state: InputState }
}

