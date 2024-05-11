use bevy::{asset::AssetLoader, prelude::*};

use crate::custom::assets::app_assets_path::AppAssetsPath;

use super::AppAssetsState;

#[derive(Default, Resource)]
pub struct AppAssetsPathHandle(pub Handle<AppAssetsPath>);

pub fn check_assets_path_ready(
    asset_server: Res<AssetServer>,
    app_assets_path_handle: Res<AppAssetsPathHandle>,
    mut assets_state: ResMut<NextState<AppAssetsState>>,
) {
    if asset_server.is_loaded_with_dependencies(&app_assets_path_handle.0) {
        assets_state.set(AppAssetsState::Loading);
    }
}

pub fn load_assets_path(
    asset_server: Res<AssetServer>,
    mut app_assets_path_handle: ResMut<AppAssetsPathHandle>,
) {
    app_assets_path_handle.0 = asset_server.load("config/app_assets_path.toml");
}
