use godot::classes::InputEvent;
use godot::classes::InputEventMouseMotion;
use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::classes::Node2D;
use godot::classes::INode2D;
use godot::classes::Node;
use godot::classes::Sprite2D;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Piece {

    #[export]
    facing_white: bool,
    following_mouse: bool,

    #[export]
    white_sprite: Option<Gd<Sprite2D>>,
    #[export]
    black_sprite: Option<Gd<Sprite2D>>,

    previous_position: Vector2,

    base: Base<Node2D>
}

#[godot_api]
impl INode2D for Piece {

    fn init(base: Base<Node2D>) -> Self {
        Self {
            facing_white: true,
            following_mouse: false,
            white_sprite: None,
            black_sprite: None,
            previous_position: Vector2::ZERO,
            base
        }
    }

    fn ready(&mut self) {
        self.previous_position = self.base().get_position();
        self.enable_white();
        if !self.facing_white {self.enable_black();};
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.following_mouse {
            if let Ok(e) = event.clone().try_cast::<InputEventMouseMotion>() {
                self.base_mut().set_position(e.get_position());
            }
        }
    }
}

#[godot_api]
pub impl Piece {

    #[func]
    pub fn click(&mut self, _: Gd<Node>, event: Gd<InputEvent>, _: i32) {
        if event.is_action_pressed("LeftClick") {
            self.following_mouse = !self.following_mouse;
            if self.following_mouse {
                self.previous_position = self.base().get_position();
            }
        } else if event.is_action_pressed("RightClick") && self.following_mouse {
            self.following_mouse = false;
            let pos = self.previous_position;
            self.base_mut().set_position(pos);
        } else if event.is_action_pressed("RightClick") && !self.following_mouse {
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