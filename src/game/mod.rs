use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod map;
mod character;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(map::MapPlugin)
            .add_startup_system(startup)
            .add_startup_system(character::spawn_character::<770, -890, -1>)
            .add_startup_system(character::spawn_character::<-190, -410, 1>)
            .add_system(character::animate_sprite)
            .add_system(highlight_mouse_tile)
            .register_type::<character::AnimationTimer>();
    }
}

fn startup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_xyz(265.0, -655.0, 1000.0 - 0.1),
        ..OrthographicCameraBundle::new_2d()
    });
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
