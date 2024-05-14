use bevy::prelude::*;

pub fn create_button<T>(
    parent: &mut ChildBuilder,
    button_image: Handle<Image>,
    button_style: Style,
    button_label: T,
    slicer: TextureSlicer,
    icon: Option<Handle<Image>>,
    icon_style: Option<Style>,
    text: &str,
    text_style: TextStyle,
) where
    T: Component,
{
    parent
        .spawn((
            ButtonBundle {
                style: button_style,
                image: UiImage::new(button_image),
                ..Default::default()
            },
            ImageScaleMode::Sliced(slicer),
            button_label,
        ))
        .with_children(|parent| {
            if let Some(icon) = icon {
                let icon_style = icon_style.unwrap_or_default();
                parent.spawn(ImageBundle {
                    image: UiImage::new(icon),
                    style: icon_style,
                    ..Default::default()
                });
            }

            parent.spawn(TextBundle::from_section(text, text_style));
        });
}
