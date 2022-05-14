use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::ldtk_pixel_coords_to_translation};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Uid(0))
            .insert_resource(LdtkSettings {
                set_clear_color: SetClearColor::FromLevelBackground,
                ..Default::default()
            })
            .add_startup_system(setup)
            .register_ldtk_int_cell::<CliffBundle>(1)
            .register_ldtk_entity::<PlayerSpawnBundle>("PlayerSpawn");
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 0.5;
    commands.spawn_bundle(camera);
    asset_server.watch_for_changes().unwrap();
    let ldtk_handle = asset_server.load("desert.ldtk");
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
struct CliffBundle {}

#[derive(Copy, Clone, PartialEq, Debug, Default, Component)]
pub struct PlayerSpawn {
    pub translation: Vec2,
    pub width: f32,
    pub height: f32,
}

impl LdtkEntity for PlayerSpawn {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let translation = ldtk_pixel_coords_to_translation(
            entity_instance.px,
            layer_instance.c_hei * layer_instance.grid_size,
        );
        PlayerSpawn {
            translation,
            width: entity_instance.width as f32,
            height: entity_instance.height as f32,
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
struct PlayerSpawnBundle {
    #[ldtk_entity]
    player_spawn: PlayerSpawn,
}
