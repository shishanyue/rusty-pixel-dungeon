use bevy_tweening::Lens;
use rusty_pixel_dungeon_server::Visibility;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UiVisibilityLens {
    /// Start position.
    pub start: Visibility,
    /// End position.
    pub end: Visibility,
}


impl Lens<Visibility> for UiVisibilityLens {
    fn lerp(&mut self, target: &mut Visibility, _: f32) {
        *target = self.end;
    }
}