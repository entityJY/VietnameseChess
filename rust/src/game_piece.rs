use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::classes::{Area2D, IArea2D, InputEvent, Sprite2D, ShapeCast2D};

use crate::node::BoardNode;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Piece {

    #[export]
    facing_white: bool,

    #[export]
    white_sprite: Option<Gd<Sprite2D>>,
    #[export]
    black_sprite: Option<Gd<Sprite2D>>,

    #[export]
    shape_cast: Option<Gd<ShapeCast2D>>,

    position_id: i64,
    hover_id: i64,

    base: Base<Area2D>
}

#[godot_api]
impl IArea2D for Piece {

    fn init(base: Base<Area2D>) -> Self {
        Self {
            facing_white: true,
            white_sprite: None,
            black_sprite: None,
            position_id: 0,
            hover_id: 0,
            shape_cast: None,
            base
        }
    }

    fn ready(&mut self) {
        self.enable_white();
        if !self.facing_white {self.enable_black();}

        self.signals().input_event().connect_self(
            |this, viewport, event, shape_idx| 
            {this.on_click(viewport, event, shape_idx);}
        );
        self.signals().area_entered().connect_self(
            |this, area|
            {this.hover_over_node(area);}
        );

        self.base_mut().call_deferred("update_node_id", &[]);
    }
}

#[godot_api]
pub impl Piece {

    #[signal]
    pub fn picked_up(piece: Gd<Piece>);

    pub fn on_click(&mut self, _viewport: Gd<Node>, event: Gd<InputEvent>, _shape_idx: i64) {
        if event.is_action_pressed("LeftClick") {
            let this_piece = &self.to_gd();
            self.signals().picked_up().emit(this_piece);
        }
    }

    fn hover_over_node(&mut self, area: Gd<Area2D>) {
        match area.try_cast::<BoardNode>() {
            Ok(node) => {self.hover_id = node.bind().get_id()},
            Err(_) => {}
        }
    }

    pub fn get_hover_id(&self) -> i64 {
        self.hover_id
    }

    pub fn is_white(&self) -> bool {
        self.facing_white
    }

    pub fn get_position_id(&self) -> i64 {
        self.position_id
    }

    #[func]
    pub fn update_node_id(&mut self) {
        self.position_id = self.shape_cast.as_ref().expect("No shape cast attached!").get_collider(0).expect("No object collision found").try_cast::<BoardNode>().expect("Object not board node!").bind().get_id();
        self.hover_id = self.position_id;
    }

    pub fn get_collisions(&self) -> Array<Gd<Area2D>> {
        let num = self.shape_cast.as_ref().expect("No shape cast attached!").get_collision_count();
        let mut collider_array: Array<Gd<Area2D>> = Array::new();
        for i in 0..num {
            let x = self.shape_cast.as_ref().unwrap().get_collider(i).unwrap().try_cast::<Area2D>().expect("Object is not an Area2D");
            collider_array.push(&x);
        }
        collider_array
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