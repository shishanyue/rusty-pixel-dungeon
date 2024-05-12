pub mod app_assets_path;
pub mod dynamic_image;
pub mod raw;

use bevy::prelude::*;

use self::{app_assets_path::AppAssetsPath, dynamic_image::DynamicImage};

use super::{handle::{app_font::AppFontHandle, app_image::AppImageHandle, assets_path::{check_assets_path_ready, load_assets_path, AppAssetsPathHandle}}, resource::init_app_res};



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
    pub app_image: AppImageHandle,
    pub app_font: AppFontHandle,

}

pub struct CustomAssetsPlugin;

impl Plugin for CustomAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AppAssetsPath>()
            .init_asset::<DynamicImage>()
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
        app_image: AppImageHandle {
            banners: asset_server.load(&app_assets_path.image.banners),
            chrome: asset_server.load(&app_assets_path.image.chrome),
            icons: asset_server.load(&app_assets_path.image.icons),
        },

        app_font: AppFontHandle {
            bold: asset_server.load(&app_assets_path.font.bold),
            extra_light: asset_server.load(&app_assets_path.font.extra_light),
            heavy: asset_server.load(&app_assets_path.font.heavy),
            light: asset_server.load(&app_assets_path.font.light),
            medium: asset_server.load(&app_assets_path.font.medium),
            normal: asset_server.load(&app_assets_path.font.normal),
            regular: asset_server.load(&app_assets_path.font.regular),
            fusion_pixel: asset_server.load(&app_assets_path.font.fusion_pixel),
        },
    });
}

fn check_assets_ready(
    asset_server: Res<AssetServer>,
    app_assets_handles: Res<AppAssetsHandles>,
    mut assets_state: ResMut<NextState<AppAssetsState>>,
) {
    if     asset_server.is_loaded_with_dependencies(&app_assets_handles.app_image.banners)
        && asset_server.is_loaded_with_dependencies(&app_assets_handles.app_image.chrome)
        && asset_server.is_loaded_with_dependencies(&app_assets_handles.app_image.icons)
        && asset_server.is_loaded_with_dependencies(&app_assets_handles.app_font.bold)
        && asset_server.is_loaded_with_dependencies(&app_assets_handles.app_font.extra_light)
        && asset_server.is_loaded_with_dependencies(&app_assets_handles.app_font.fusion_pixel)
        && asset_server.is_loaded_with_dependencies(&app_assets_handles.app_font.heavy)
        && asset_server.is_loaded_with_dependencies(&app_assets_handles.app_font.light)
        && asset_server.is_loaded_with_dependencies(&app_assets_handles.app_font.medium)
        && asset_server.is_loaded_with_dependencies(&app_assets_handles.app_font.normal)
        && asset_server.is_loaded_with_dependencies(&app_assets_handles.app_font.regular)
    {
        assets_state.set(AppAssetsState::Loaded);
    }
}
