use bevy::app::App;

use crate::{level::level_project::LevelProject, room::room_project::RoomProject};


pub trait AppExt {
    fn init_level<T: LevelProject>(&mut self) -> &mut Self;
    fn init_room<T: RoomProject>(&mut self) -> &mut Self;
}

impl AppExt for App {
    fn init_level<T: LevelProject>(&mut self) -> &mut Self {
        let level = T::default();
        level.build(self);
        self.insert_resource(level)
    }

    fn init_room<T: RoomProject>(&mut self) -> &mut Self {
        let room = T::default();
        room.build(self);
        self.insert_resource(room)
    }
}
