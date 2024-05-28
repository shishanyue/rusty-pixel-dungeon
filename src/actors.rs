pub mod hero;
use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use self::hero::{ActiveHero, SelectedHero};

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedHero::Unlocked(hero::HeroType::Rogue))
        .add_systems(Update, (move_player_from_input,translate_grid_coords_entities));
    }
}

fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation =
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(16))
                .extend(transform.translation.z);
    }
}

fn move_player_from_input(
    mut players: Query<&mut GridCoords, With<ActiveHero>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let movement_direction = if input.just_pressed(KeyCode::KeyW) {
        GridCoords::new(0, 1)
    } else if input.just_pressed(KeyCode::KeyA) {
        GridCoords::new(-1, 0)
    } else if input.just_pressed(KeyCode::KeyS) {
        GridCoords::new(0, -1)
    } else if input.just_pressed(KeyCode::KeyD) {
        GridCoords::new(1, 0)
    } else {
        return;
    };

    for mut player_grid_coords in players.iter_mut() {
        let destination = *player_grid_coords + movement_direction;
        *player_grid_coords = destination;
    }
}
