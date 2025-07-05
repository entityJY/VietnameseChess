use godot::prelude::*;

struct MyExtension;

// don't forget to do: mod {file_name};
mod utils;
mod game_piece;
mod node;
mod game_manager;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}



// rustup toolchain install nightly
// rustup component add rust-src --toolchain nightly
// rustup target add wasm32-unknown-emscripten --toolchain nightly
// source ../../../emsdk/emsdk_env.sh
// cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten --release