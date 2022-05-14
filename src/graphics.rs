use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_graphics);
    }
}

pub struct PlayerImage {
    pub handle: Handle<Image>,
}

fn load_graphics(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerImage {
        handle: asset_server.load("player.png"),
    })
}
