use bevy::{prelude::*, render::camera::Camera2d};
use bevy_inspector_egui::Inspectable;
use rand::Rng;

use crate::{graphics::PlayerImage, level::PlayerSpawn};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player)
            .add_system(kill_player.before(spawn_player))
            .add_system(player_movement)
            .add_system(camera_follow.after(player_movement));
    }
}

#[derive(Default, Component, Inspectable)]
pub struct Player {
    walk_speed: f32,
}

fn spawn_player(
    mut commands: Commands,
    player_query: Query<&Player>,
    player_spawn_query: Query<&PlayerSpawn>,
    player_image: Res<PlayerImage>,
) {
    if !player_query.is_empty() {
        return;
    }
    if let Ok(player_spawn) = player_spawn_query.get_single() {
        println!("Spawning player");
        let x = player_spawn.translation.x;
        let y = player_spawn.translation.y;
        let spawn_x = rand::thread_rng().gen_range(x..x + player_spawn.width);
        let spawn_y = rand::thread_rng().gen_range(y - player_spawn.height..y);
        commands
            .spawn_bundle(SpriteBundle {
                texture: player_image.handle.clone(),
                transform: Transform::from_xyz(spawn_x, spawn_y, 800.),
                ..default()
            })
            .insert(Name::new("Player"))
            .insert(Player { walk_speed: 100.0 });
    }
}

fn kill_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::K) {
        if let Ok(player) = player_query.get_single() {
            println!("Killing player");
            commands.entity(player).despawn_recursive();
        }
    }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((player, mut transform)) = player_query.get_single_mut() {
        let speed = player.walk_speed * time.delta_seconds();
        let mut velocity = Vec2::new(0.0, 0.0);
        if keyboard.pressed(KeyCode::W) {
            velocity.y += speed;
        }
        if keyboard.pressed(KeyCode::S) {
            velocity.y -= speed;
        }
        if keyboard.pressed(KeyCode::A) {
            velocity.x -= speed;
        }
        if keyboard.pressed(KeyCode::D) {
            velocity.x += speed;
        }
        if velocity == Vec2::ZERO {
            return;
        }
        transform.translation += velocity.extend(0.0);
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera2d>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation = player_transform.translation;
    }
}
