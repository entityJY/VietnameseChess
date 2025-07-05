use godot::prelude::*;
use godot::classes::{InputEvent, InputEventMouseButton, InputEventMouseMotion, InputEventScreenTouch};


pub enum InputKind {
    MouseMotion(Gd<InputEventMouseMotion>),
    MouseButton(Gd<InputEventMouseButton>),
    ScreenTouch(Gd<InputEventScreenTouch>),
    Other(Gd<InputEvent>),
}

impl From<Gd<InputEvent>> for InputKind {
    fn from(event: Gd<InputEvent>) -> Self {
        if let Ok(e) = event.clone().try_cast::<InputEventMouseMotion>() {
            Self::MouseMotion(e)
        } else if let Ok(e) = event.clone().try_cast::<InputEventMouseButton>() {
            Self::MouseButton(e)
        } else if let Ok(e) = event.clone().try_cast::<InputEventScreenTouch>() {
            Self::ScreenTouch(e)
        } else {
            Self::Other(event)
        }
    }
}

/* usage
match InputKind::from(event) {
    InputKind::MouseMotion(e) => {},
    InputKind::MouseButton(e) => {},
    InputKind::ScreenTouch(e) => {},
    InputKind::Other(e) => {},
}
*/