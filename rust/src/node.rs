use godot::prelude::*;
use godot::classes::Area2D;
use godot::classes::IArea2D;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct BoardNode {
    #[export]
    id: i64,
    base: Base<Area2D>
}

#[godot_api]
impl IArea2D for BoardNode {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            id: 0,
            base
        }
    }
}