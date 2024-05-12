use bevy::prelude::*;

#[derive(Default,Debug,Clone)]
pub struct AppFontHandle{
    pub bold:Handle<Font>,
    pub extra_light:Handle<Font>,
    pub heavy:Handle<Font>,
    pub light:Handle<Font>,
    pub medium:Handle<Font>,
    pub normal:Handle<Font>,
    pub regular:Handle<Font>,
    pub fusion_pixel:Handle<Font>
}