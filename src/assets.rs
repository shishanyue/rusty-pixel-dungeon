use bevy::prelude::*;

use crate::system::SystemStatus;

pub struct AssetsPlugin;

#[derive(Default, Resource)]
pub struct AppRes {
    pub banners: BannersRes,
}

#[derive(Default)]
pub struct BannersRes {
    pub texture_handle: Handle<Image>,
    pub pixel_dungeon_atlas: TextureAtlas,
    pub pixel_dungeon_signs_atlas: TextureAtlas,
}

#[derive(Component)]
pub struct PixelDungeon;
#[derive(Component)]
pub struct PixelDungeonSigns;


impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppRes>()
            .add_systems(PreStartup, load_assets);
    }
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut app_res: ResMut<AppRes>,
    mut system_status: ResMut<SystemStatus>,
    mut texture_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let banners_handle = asset_server.load("interfaces/banners.png");

    let mut banners_layout = TextureAtlasLayout::new_empty(Vec2::new(256., 256.));
    let pixel_dungeon_index = banners_layout.add_texture(Rect::new(0., 0., 132., 90.));
    let pixel_dungeon_signs_index = banners_layout.add_texture(Rect::new(132., 0., 256., 90.));

    let banners_layout_handle = texture_layouts.add(banners_layout);

    app_res.banners.pixel_dungeon_atlas = TextureAtlas {
        layout: banners_layout_handle.clone(),
        index: pixel_dungeon_index,
    };

    app_res.banners.pixel_dungeon_signs_atlas = TextureAtlas {
        layout: banners_layout_handle.clone(),
        index: pixel_dungeon_signs_index,
    };

    app_res.banners.texture_handle = banners_handle;

    system_status.inited_assets = true;
}
