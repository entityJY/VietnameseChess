use godot::prelude::*;

struct MyExtension;

// don't forget to do: mod {file_name};
mod game_piece;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
