pub mod app_res;
pub mod assets_path;
pub mod banners;
pub mod chrome;

use crate::custom::assets::{app_assets_path::AppAssetsPath, dynamic_image::DynamicImage};

use self::{
    app_res::{init_app_res, AppRes},
    assets_path::{check_assets_path_ready, load_assets_path, AppAssetsPathHandle},
};
use bevy::prelude::*;

pub struct AssetsPlugin;

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum AppAssetsState {
    #[default]
    None,
    Loading,
    Loaded,
    Failed,
}

#[derive(Debug, Resource)]
pub struct AppAssetsHandles {
    pub banners: Handle<Image>,
    pub chrome: Handle<DynamicImage>,
}

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppAssetsPathHandle>()
            .init_resource::<AppRes>()
            .init_state::<AppAssetsState>()
            .add_systems(PreStartup, load_assets_path)
            .add_systems(
                PreUpdate,
                check_assets_path_ready.run_if(in_state(AppAssetsState::None)),
            )
            .add_systems(OnEnter(AppAssetsState::Loading), load_assets)
            .add_systems(
                PreUpdate,
                check_assets_ready.run_if(in_state(AppAssetsState::Loading)),
            )
            .add_systems(OnEnter(AppAssetsState::Loaded), init_app_res);
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    app_assets_path_handle: Res<AppAssetsPathHandle>,
    app_assets_path: Res<Assets<AppAssetsPath>>,
) {
    let app_assets_path = app_assets_path.get(&app_assets_path_handle.0).unwrap();
    commands.insert_resource(AppAssetsHandles {
        banners: asset_server.load(&app_assets_path.banners),
        chrome: asset_server.load(&app_assets_path.chrome),
    });
}

fn check_assets_ready(
    asset_server: Res<AssetServer>,
    app_assets_handles: Res<AppAssetsHandles>,
    mut assets_state: ResMut<NextState<AppAssetsState>>,
) {
    if asset_server.is_loaded_with_dependencies(&app_assets_handles.banners)
        && asset_server.is_loaded_with_dependencies(&app_assets_handles.chrome)
    {
        assets_state.set(AppAssetsState::Loaded);
    }
}
