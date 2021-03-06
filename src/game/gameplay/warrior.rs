use super::attribute::*;
use super::weapon::Weapon;
use super::Map;
use super::MapPosition;
use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(Default, Component)]
pub struct Warrior;

#[derive(Default, Bundle)]
pub struct WarriorBundle {
    _c: Warrior,
    name: Name,
    position: MapPosition,
    health: Health,
    weapon: Weapon,
    action_points: ActionPoints,
    movement_points: MovementPoints,
    #[bundle]
    sprite: SpriteSheetBundle,
    animation_timer: AnimationTimer,
}

impl WarriorBundle {
    pub fn new(
        name: String,
        position: MapPosition,
        weapon: &Weapon,
        flip: f32,
        texture_atlas_handle: &Handle<TextureAtlas>,
    ) -> Self {
        WarriorBundle {
            name: Name::new(name),
            position,
            health: Health(Attribute {
                value: 1500,
                max: 1500,
            }),
            action_points: ActionPoints(Attribute { value: 8, max: 8 }),
            movement_points: MovementPoints(Attribute { value: 5, max: 5 }),
            weapon: *weapon,
            animation_timer: AnimationTimer(Timer::from_seconds(0.15, true)),
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0))
                    .with_scale(Vec3::new(2.0 * flip, 2.5, 1.0)),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

#[derive(AssetCollection)]
pub struct WarriorAssets {
    #[asset(key = "animations.ella")]
    pub idle: Handle<TextureAtlas>,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct AnimationTimer(pub Timer); // TODO maybe improve this thing to support multiple animations (idle, run, attack...)

/// Animate the sprite based on the AnimationTimer
pub fn animate_warrior_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

/// Update the warrior's Transform based on it's MapPosition
pub fn update_warrior_world_position(
    map_query: Query<&Map>,
    mut warrior_query: Query<(&mut Transform, &MapPosition), (With<Warrior>, Changed<MapPosition>)>,
) {
    if map_query.is_empty() {
        return;
    }
    if warrior_query.is_empty() {
        return;
    }

    let map = map_query.single();
    let obstacle_layer_id = map.obstacle_layer;
    let map_width = map.width;
    let map_height = map.height;
    let tile_width = map.tile_width as f32;
    let tile_height = map.tile_height as f32;

    for (mut transform, position) in warrior_query.iter_mut() {
        transform.translation = position.to_xyz(
            obstacle_layer_id,
            map_width,
            map_height,
            tile_width,
            tile_height,
        );
    }
}
