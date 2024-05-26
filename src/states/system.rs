use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct SystemStatus {
    pub inited_assets: bool,
}

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum SystemState {
    #[default]
    Loading,
    Loaded,
}
pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SystemState>()
            .init_resource::<SystemStatus>()
            .add_systems(
                Update,
                (|mut system_state: ResMut<NextState<SystemState>>,
                  system_status: Res<SystemStatus>| {
                    if system_status.inited_assets {
                        system_state.set(SystemState::Loaded);
                        info!("Loaded");
                    }
                })
                .run_if(in_state(SystemState::Loading)),
            );
    }
}
