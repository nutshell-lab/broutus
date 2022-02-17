use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod character;
mod gameplay;
mod map;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(map::MapPlugin)
            .add_startup_system(spawn_main_camera)
            .add_startup_system(setup_gameplay)
            .add_system(character::animate_sprite)
            .add_system(gameplay::debug_ui_turn)
            .add_system(highlight_mouse_tile)
            .register_type::<character::AnimationTimer>();
    }
}

fn spawn_main_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_xyz(265.0, -655.0, 1000.0 - 0.1),
        ..OrthographicCameraBundle::new_2d()
    });
}

fn setup_gameplay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("characters/knight_idle.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 15, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let knight_blue = commands
        .spawn_bundle(character::CharacterBundle::new(
            "Knight Blue".to_string(),
            Vec2::new(770.0, -890.0),
            -1.0,
            &texture_atlas_handle,
        ))
        .id();
    let knight_red = commands
        .spawn_bundle(character::CharacterBundle::new(
            "Knight Red".to_string(),
            Vec2::new(-190.0, -410.0),
            1.0,
            &texture_atlas_handle,
        ))
        .id();

    commands.insert_resource(gameplay::Turn {
        order: vec![knight_blue, knight_red],
        ..Default::default()
    })
}

fn highlight_mouse_tile(
    position: Res<map::MouseMapPosition>,
    previous_position: Res<map::PreviousMouseMapPosition>,
    mut map_query: map::MapQuery,
    mut tile_query: Query<&mut map::Tile>,
) {
    if position.is_changed() {
        if let Some(position) = position.0 {
            let color = if map::is_obstacle(&mut map_query, position) {
                Color::GRAY
            } else {
                Color::SEA_GREEN
            };

            map::highlight_tile(&mut map_query, &mut tile_query, position, color);
        }
    }

    if previous_position.is_changed() {
        if let Some(position) = previous_position.0 {
            map::highlight_tile(&mut map_query, &mut tile_query, position, Color::WHITE);
        }
    }
}
