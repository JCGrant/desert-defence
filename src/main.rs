use bevy::prelude::*;

mod debug;
mod graphics;
mod level;
mod player;

fn main() {
    App::new()
        .insert_resource(bevy::log::LogSettings {
            filter: "bevy_ecs_tilemap::render::pipeline=error,wgpu=error".to_string(),
            level: bevy::log::Level::INFO,
        })
        .insert_resource(ClearColor(Color::hex("FFCF5E").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(graphics::GraphicsPlugin)
        .add_plugin(level::LevelPlugin)
        .add_plugin(player::PlayerPlugin)
        .run();
}
