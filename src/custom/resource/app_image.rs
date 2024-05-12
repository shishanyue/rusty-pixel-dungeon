pub mod banners;
pub mod chrome;
pub mod icons;

use self::{banners::BannersResource, chrome::ChromeResource, icons::IconsResource};

#[derive(Default)]
pub struct AppImageResource {
    pub banners: BannersResource,
    pub chrome: ChromeResource,
    pub icons: IconsResource
}
