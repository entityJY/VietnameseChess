use godot::prelude::*;

struct MyExtension;

// don't forget to do: mod {file_name};
mod utils;
mod game_piece;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}


// cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten --release