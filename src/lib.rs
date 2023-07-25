//! This crate simplifies the process of creating widgets in bevy using a simple extensible DSL.

mod widgets;
#[cfg(feature = "class_helpers")]
pub mod class_helpers;

use bevy_text::TextStyle;
pub use widgets::*;
use bevy_ui::node_bundles::{NodeBundle, ImageBundle, TextBundle, ButtonBundle};
use bevy_ecs::entity::Entity;
use bevy_ecs::system::EntityCommands;
use bevy_ecs::bundle::Bundle;
use bevy_hierarchy::{ChildBuilder, BuildChildren};


/// Wrapper for [`ChildBuilder`] that also propogates data for the children that need it.
// It has enough ' for a lifetime ;)
pub struct UiChildBuilder<'a, 'b, 'c, 'd, AssetData> {
    builder: &'a mut ChildBuilder<'b, 'c, 'd>,
    asset_data: &'a AssetData
}

impl<'a, 'b, 'c, 'd, AssetData> UiChildBuilder<'a, 'b, 'c, 'd, AssetData> {
    pub fn spawn(&mut self, bundle: impl Bundle) -> UiEntityCommands<'a, 'b, 'c, '_, AssetData> {
        let commands: EntityCommands<'b, 'c, '_> = self.builder.spawn(bundle);
        UiEntityCommands {
            asset_data: self.asset_data,
            commands
        }
    }
    pub fn asset_data(&self) -> &AssetData { self.asset_data }
}

/// Wrapper for [`EntityCommands`] that also propagates data for the children that need it.
pub struct UiEntityCommands<'a, 'b, 'c, 'd, AssetData> {
    commands: EntityCommands<'b, 'c, 'd>,
    asset_data: &'a AssetData
}

impl<'a, 'b, 'c, 'd, AssetData> UiEntityCommands<'a, 'b, 'c, 'd, AssetData> {
    pub fn id(&self) -> Entity {
        self.commands.id()
    }
    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.commands.insert(bundle);
        self
    }
    pub fn with_children(mut self, spawn_children: impl FnOnce(&mut UiChildBuilder<AssetData>)) -> Self {
        self.commands.with_children(|builder| {
            let mut ui_builder = UiChildBuilder {
                asset_data: self.asset_data,
                builder
            };
            spawn_children(&mut ui_builder);
        });
        self
    }
}

/// Something that can overwrite a value, typically a node bundle.
pub trait Class<AssetData, B> {
    fn apply(self, data: &AssetData, b: &mut B);
}

impl<AssetData, T> Class<AssetData, T> for () {
    fn apply(self, _a: &AssetData, _b: &mut T) {}
}

impl<F, AssetData, B> Class<AssetData, B> for F
where
    F: FnOnce(&AssetData, &mut B)
{
    fn apply(self, a: &AssetData, b: &mut B) {
        self(a, b);
    }
}

impl<F1, F2, AssetData, B> Class<AssetData, B> for (F1, F2)
where
    F1: Class<AssetData, B>,
    F2: Class<AssetData, B>,
{
    fn apply(self, a: &AssetData, b: &mut B) {
        self.0.apply(a, b);
        self.1.apply(a, b);
    }
}

impl<F1, F2, F3, AssetData, B> Class<AssetData, B> for (F1, F2, F3)
where
    F1: Class<AssetData, B>,
    F2: Class<AssetData, B>,
    F3: Class<AssetData, B>,
{
    fn apply(self, a: &AssetData, b: &mut B) {
        self.0.apply(a, b);
        self.1.apply(a, b);
        self.2.apply(a, b);
    }
}

impl<F1, F2, F3, F4, AssetData, B> Class<AssetData, B> for (F1, F2, F3, F4)
where
    F1: Class<AssetData, B>,
    F2: Class<AssetData, B>,
    F3: Class<AssetData, B>,
    F4: Class<AssetData, B>,
{
    fn apply(self, a: &AssetData, b: &mut B) {
        self.0.apply(a, b);
        self.1.apply(a, b);
        self.2.apply(a, b);
        self.3.apply(a, b);
    }
}

impl<AssetData> Class<AssetData, NodeBundle> for NodeBundle {
    fn apply(self, _a: &AssetData, b: &mut NodeBundle) {
        *b = self;
    }
}

impl<AssetData> Class<AssetData, ImageBundle> for ImageBundle {
    fn apply(self, _a: &AssetData, b: &mut ImageBundle) {
        *b = self;
    }
}

impl<AssetData> Class<AssetData, ButtonBundle> for ButtonBundle {
    fn apply(self, _a: &AssetData, b: &mut ButtonBundle) {
        *b = self;
    }
}

impl<AssetData> Class<AssetData, TextBundle> for TextBundle {
    fn apply(self, _a: &AssetData, b: &mut TextBundle) {
        *b = self;
    }
}

impl<AssetData> Class<AssetData, TextStyle> for TextStyle {
    fn apply(self, _a: &AssetData, b: &mut TextStyle) {
        *b = self;
    }
}

/// Adds a helper method to [`Entity`] that allows it to be sent to an [`Option`][`Entity`]
/// ergonomically.
pub trait EntityWriter {
    fn set(self, entity: &mut Option<Entity>);
    fn push(self, destination: &mut Vec<Entity>);
}

impl EntityWriter for Entity {
    /// Copies this entity into an Option.
    fn set(self, entity: &mut Option<Entity>) {
        *entity = Some(self);
    }
    /// Pushes a copy of this Entity into a Vec.
    fn push(self, entities: &mut Vec<Entity>) {
        entities.push(self);
    }
}
