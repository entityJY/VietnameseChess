use godot::classes::InputEvent;
use godot::prelude::*;
use godot::classes::Node2D;
use godot::classes::INode2D;
use godot::classes::Node;
use godot::classes::Sprite2D;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Piece {
    facing_white: bool,
    #[export]
    white_sprite: Option<Gd<Sprite2D>>,
    #[export]
    black_sprite: Option<Gd<Sprite2D>>,

    base: Base<Node2D>
}

#[godot_api]
impl INode2D for Piece {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            facing_white: true,
            white_sprite: None,
            black_sprite: None,
            base
        }
    }
}

#[godot_api]
pub impl Piece {

    #[func]
    pub fn test(&mut self, _: Gd<Node>, event: Gd<InputEvent>, _: i32) {
        if event.is_action_pressed("LeftClick") {
            self.flip_piece();
        }
    }

    pub fn flip_piece(&mut self) {
        if self.facing_white { self.enable_black(); self.facing_white=false } else { self.enable_white(); self.facing_white=true }
    }

    fn enable_white(&mut self) {
        self.white_sprite.as_mut().expect("No white sprite chosen!").set_visible(true);
        self.black_sprite.as_mut().expect("No black sprite chosen!").set_visible(false);
    }

    fn enable_black(&mut self) {
        self.white_sprite.as_mut().expect("No white sprite chosen!").set_visible(false);
        self.black_sprite.as_mut().expect("No black sprite chosen!").set_visible(true);
    }

}