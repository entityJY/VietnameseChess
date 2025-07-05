use godot::prelude::*;
use godot::classes::{INode2D, InputEvent, Node2D};

use crate::game_piece::Piece;
use crate::utils::InputKind;


#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Manager {
    held_piece: Option<Gd<Piece>>,
    previous_node_id: i64,
    is_white_turn: bool,
    base: Base<Node2D>
}

#[godot_api]
impl INode2D for Manager {

    fn init(base: Base<Node2D>) -> Self {
        Self {
            held_piece: None,
            is_white_turn: true,
            previous_node_id: 0,
            base
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        match InputKind::from(event) {
            InputKind::MouseMotion(e) => {
                if let Some(piece) = &mut self.held_piece {
                    piece.set_position(e.get_position());
                }
            },
            InputKind::MouseButton(_e) => {},
            InputKind::ScreenTouch(_e) => {},
            InputKind::Other(_e) => {},
        }
    }

    fn ready(&mut self) {
        let pieces = self.base().get_node_as::<Node2D>("Pieces").get_children();
        for i in 0..pieces.len() {
            if let Ok(piece) =  pieces.get(i).unwrap().try_cast::<Piece>() {
                piece.signals().picked_up().connect_other(
                    self, 
                    |this, piece| {this.handle_piece_click(piece);}
                );
            }
        }
    }

    fn process(&mut self, _delta: f64) {
        
    }

}

#[godot_api]
impl Manager {

    fn handle_piece_click(&mut self, mut piece: Gd<Piece>) {
        if self.held_piece.is_none() {
            if (self.is_white_turn && piece.bind().is_white()) || (!self.is_white_turn && !piece.bind().is_white()) {
                self.previous_node_id = piece.bind().get_position_id();
                self.held_piece = Some(piece);
                self.held_piece.as_mut().unwrap().set_z_index(2);
            }
        } else {
            let areas = piece.bind().get_collisions();

            let mut placing_allowed = true;
            if areas.len() == 0 {placing_allowed = false;}
            for i in 0..areas.len() {
                match areas.get(i).unwrap().try_cast::<Piece>() {
                    Ok(_) => {placing_allowed = false;},
                    Err(_) => {}
                }
            }
            if placing_allowed == true {
                self.held_piece = None;
                piece.bind_mut().update_node_id();
                self.held_piece.as_mut().unwrap().set_z_index(1);
            }
        }
    }

}