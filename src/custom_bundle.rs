use bevy::{prelude::*, ui::{ContentSize, FocusPolicy}};


#[derive(Bundle, Debug, Default)]
pub struct AtlasButtonBundle{
        /// Describes the logical size of the node
        pub node: Node,
        /// Marker component that signals this node is a button
        pub button: Button,
        /// Styles which control the layout (size and position) of the node and it's children
        /// In some cases these styles also affect how the node drawn/painted.
        pub style: Style,
        /// The calculated size based on the given image
        pub calculated_size: ContentSize,
        /// Describes whether and how the button has been interacted with by the input
        pub interaction: Interaction,
        /// Whether this node should block interaction with lower nodes
        pub focus_policy: FocusPolicy,
        /// The background color, which serves as a "fill" for this node
        ///
        /// When combined with `UiImage`, tints the provided image.
        pub background_color: BackgroundColor,
        /// The color of the Node's border
        pub border_color: BorderColor,
        /// The image of the node
        pub image: UiImage,
        /// A handle to the texture atlas to use for this Ui Node
        pub texture_atlas: TextureAtlas,
        /// The transform of the node
        ///
        /// This component is automatically managed by the UI layout system.
        /// To alter the position of the `ButtonBundle`, use the properties of the [`Style`] component.
        pub transform: Transform,
        /// The global transform of the node
        ///
        /// This component is automatically updated by the [`TransformPropagate`](`bevy_transform::TransformSystem::TransformPropagate`) systems.
        pub global_transform: GlobalTransform,
        /// Describes the visibility properties of the node
        pub visibility: Visibility,
        /// Inherited visibility of an entity.
        pub inherited_visibility: InheritedVisibility,
        /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
        pub view_visibility: ViewVisibility,
        /// Indicates the depth at which the node should appear in the UI
        pub z_index: ZIndex,
}