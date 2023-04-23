use bevy::prelude::*;
mod components;
mod plugins;
use plugins::{*, hello_plugin::HelloPlugin};
fn main() {
    App::new()
         .add_plugins(DefaultPlugins)
         .add_plugin(HelloPlugin)
        .run();
}
