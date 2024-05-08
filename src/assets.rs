pub mod assets_path;
pub mod banners;
pub mod chrome;

use bevy::prelude::*;

use crate::system::SystemStatus;

use self::{
    assets_path::{AppAssetsPath, AppAssetsPathLoader},
    banners::BannersRes,
    chrome::ChromeRes,
};

pub struct AssetsPlugin;

#[derive(Default, Resource)]
pub struct AppRes {
    pub banners: BannersRes,
    pub chrome: ChromeRes,
}

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AppAssetsPath>()
            .init_asset_loader::<AppAssetsPathLoader>()
            .init_resource::<AppRes>()
            .add_systems(
                PreStartup,
                (load_assets_path, load_assets.after(load_assets_path)),
            );
    }
}
fn load_assets_path(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut app_assets_paths: ResMut<Assets<AppAssetsPath>>,
) {
    let app_assets_path_handle: Handle<AppAssetsPath> =
        asset_server.load::<AppAssetsPath>("config/app_assets_path.toml");
    let app_assets_path = loop {
        match asset_server.get_load_state(app_assets_path_handle.clone()) {
            Some(state) => match state {
                bevy::asset::LoadState::Loaded => {
                    break app_assets_paths
                        .get(app_assets_path_handle)
                        .unwrap()
                        .clone()
                }
                bevy::asset::LoadState::Failed => {
                    let new = app_assets_paths.add(AppAssetsPath::default());
                    break app_assets_paths.get(new).unwrap().clone();
                }
                _ => {
                    continue;
                }
            },
            None => {
                continue;
            }
        }
    };

    commands.insert_resource(app_assets_path);
}
fn load_assets(
    asset_server: Res<AssetServer>,
    mut app_res: ResMut<AppRes>,
    mut system_status: ResMut<SystemStatus>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    //mut app_assets_path_load_error: EventReader<AssetLoadFailedEvent<AppAssetsPath>>,
    app_assets_path_res: Res<AppAssetsPath>,
) {
    BannersRes::load(
        &mut app_res,
        &app_assets_path_res,
        &asset_server,
        &mut atlas_layouts,
    );
    system_status.inited_assets = true;
}
