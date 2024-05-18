use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct SystemStatus {
    pub inited_assets: bool,
}

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SystemStatus>();
    }
}
