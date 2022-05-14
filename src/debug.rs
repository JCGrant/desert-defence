use bevy::prelude::*;
use bevy_inspector_egui::{widgets::InspectorQuery, Inspectable, InspectorPlugin};

use crate::player::Player;

#[derive(Inspectable, Default)]
struct Data {
    query: InspectorQuery<Entity, With<Player>>,
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(InspectorPlugin::<Data>::new());
        }
    }
}
