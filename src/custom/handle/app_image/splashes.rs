use bevy::prelude::*;


#[derive(Default,Debug,Clone)]
pub struct SplashesHandles {
    pub duelist: Handle<Image>,
    pub huntress: Handle<Image>,
    pub mage: Handle<Image>,
    pub rogue: Handle<Image>,
    pub warrior: Handle<Image>,
}