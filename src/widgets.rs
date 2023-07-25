use bevy_ecs::prelude::Bundle;
use bevy_text::{TextStyle, TextSection};
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Commands;
use bevy_ui::{Val, FlexWrap, Style, JustifyContent, AlignItems};
use bevy_ui::node_bundles::{NodeBundle, TextBundle, ButtonBundle, ImageBundle};
use bevy_hierarchy::BuildChildren;
use super::{Class, UiChildBuilder};


/// Spawns a [`NodeBundle`] as the root with children.
pub fn root<AssetData>(
    class: impl Class<AssetData, NodeBundle>,
    asset_data: &AssetData,
    commands: &mut Commands,
    children: impl FnOnce(&mut UiChildBuilder<AssetData>)
) -> Entity {
    rooti(class, asset_data, commands, (), children)
}

/// Spawns a [`NodeBundle`] as the root with children.
pub fn rooti<AssetData>(
    class: impl Class<AssetData, NodeBundle>,
    asset_data: &AssetData,
    commands: &mut Commands,
    extras: impl Bundle,
    children: impl FnOnce(&mut UiChildBuilder<AssetData>)
) -> Entity {
    let mut bundle = NodeBundle::default();
    class.apply(asset_data, &mut bundle);
    commands
        .spawn((bundle, extras))
        .with_children(|builder| {
            let mut builder = UiChildBuilder {
                builder,
                asset_data
            };
            children(&mut builder);
        })
        .id()
}


/// Spawns a clear [`NodeBundle`] that takes up the full space of its parent.
/// Often required for embedding other widgets after the initial widget is spawned.
pub fn blank<AssetData>(
    parent: Entity,
    class: impl Class<AssetData, NodeBundle>,
    asset_data: &AssetData,
    commands: &mut Commands,
    children: impl FnOnce(&mut UiChildBuilder<AssetData>)
) -> Entity {
    blanki(parent, class, asset_data, commands, (), children)
}

/// Spawns a clear [`NodeBundle`] that takes up the full space of its parent.
/// Often required for embedding other widgets after the initial widget is spawned.
pub fn blanki<AssetData>(
    parent: Entity,
    class: impl Class<AssetData, NodeBundle>,
    asset_data: &AssetData,
    commands: &mut Commands,
    extras: impl Bundle,
    children: impl FnOnce(&mut UiChildBuilder<AssetData>)
) -> Entity {
    commands
        .entity(parent)
        .with_children(|builder| {
            let mut bundle = NodeBundle::default();
            class.apply(asset_data, &mut bundle);
            let mut builder = UiChildBuilder {
                builder,
                asset_data
            };
            builder.spawn((bundle, extras)).with_children(children);
        })
        .id()
}

/// Spawns a [`NodeBundle`] with children.
pub fn node<AssetData>(
    class: impl Class<AssetData, NodeBundle>,
    parent: &mut UiChildBuilder<AssetData>,
    children: impl FnOnce(&mut UiChildBuilder<AssetData>)
) -> Entity {
    nodei(class, (), parent, children)
}


/// Spawns a [`NodeBundle`] with children.
pub fn nodei<AssetData>(
    class: impl Class<AssetData, NodeBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder<AssetData>,
    children: impl FnOnce(&mut UiChildBuilder<AssetData>)
) -> Entity {
    let mut bundle = NodeBundle::default();
    class.apply(parent.asset_data, &mut bundle);
    
    let mut commands = parent.spawn(bundle);
    commands.insert(extras);
    commands.with_children(children).id()
}

/// Spawns a [`TextBundle`].
pub fn text<AssetData>(
    text: impl Into<String>,
    class: impl Class<AssetData, TextBundle>,
    text_class: impl Class<AssetData, TextStyle>,
    parent: &mut UiChildBuilder<AssetData>
) -> Entity {
    texti(text, class, text_class, (), parent)
}

/// Spawns a [`TextBundle`].
pub fn texti<AssetData>(
    text: impl Into<String>,
    class: impl Class<AssetData, TextBundle>,
    text_class: impl Class<AssetData, TextStyle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder<AssetData>
) -> Entity {
    let mut bundle = TextBundle::default();
    class.apply(parent.asset_data, &mut bundle);
    let sections = &mut bundle.text.sections;
    let mut style = TextStyle::default();
    text_class.apply(parent.asset_data, &mut style);
    sections.push(TextSection {
        value: text.into(),
        style,
    });
    parent.spawn((bundle, extras)).id()
}

/// Spawns a [`ButtonBundle`] with children.
pub fn button<AssetData>(
    class: impl Class<AssetData, ButtonBundle>,
    parent: &mut UiChildBuilder<AssetData>,
    children: impl FnOnce(&mut UiChildBuilder<AssetData>)
) -> Entity {
    buttoni(class, (), parent, children)
}

/// Spawns a [`ButtonBundle`] with children.
pub fn buttoni<AssetData>(
    class: impl Class<AssetData, ButtonBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder<AssetData>,
    children: impl FnOnce(&mut UiChildBuilder<AssetData>)
) -> Entity {
    let mut bundle = ButtonBundle::default();
    class.apply(parent.asset_data, &mut bundle);
    parent
        .spawn((bundle, extras))
        .with_children(children).id()
}

/// Spawns a [`ButtonBundle`] without children.
pub fn simple_button<AssetData>(
    class: impl Class<AssetData, ButtonBundle>,
    parent: &mut UiChildBuilder<AssetData>
) -> Entity {
    simple_buttoni(class, (), parent)
}

/// Spawns a [`ButtonBundle`] without children.
pub fn simple_buttoni<AssetData>(
    class: impl Class<AssetData, ButtonBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder<AssetData>
) -> Entity {
    let mut bundle = ButtonBundle::default();
    class.apply(parent.asset_data, &mut bundle);
    parent.spawn((bundle, extras)).id()
}

/// Spawns a [`ButtonBundle`] with a single [`TextBundle`] as its child.
pub fn text_button<AssetData>(
    txt: impl Into<String>,
    class: impl Class<AssetData, ButtonBundle>,
    text_style: impl Class<AssetData, TextStyle>,
    parent: &mut UiChildBuilder<AssetData>
) -> Entity {
    text_buttoni(txt, class, text_style, (), parent)
}

/// Spawns a [`ButtonBundle`] with a single [`TextBundle`] as its child.
pub fn text_buttoni<AssetData>(
    txt: impl Into<String>,
    class: impl Class<AssetData, ButtonBundle>,
    text_style: impl Class<AssetData, TextStyle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder<AssetData>
) -> Entity {
    buttoni(class, extras, parent, |p| {
        text(txt, (), text_style, p);
    })
}

/// Spawns an [`ImageBundle`].
pub fn image<AssetData>(
    class: impl Class<AssetData, ImageBundle>,
    parent: &mut UiChildBuilder<AssetData>
) -> Entity {
    imagei(class, (), parent)
}

/// Spawns an [`ImageBundle`].
pub fn imagei<AssetData>(
    class: impl Class<AssetData, ImageBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder<AssetData>
) -> Entity {
    let mut bundle = ImageBundle::default();
    class.apply(parent.asset_data, &mut bundle);
    parent.spawn((bundle, extras)).id()
}

/// Spawns an [`ImageBundle`] with children.
pub fn image_pane<AssetData>(
    class: impl Class<AssetData, ImageBundle>,
    parent: &mut UiChildBuilder<AssetData>,
    children: impl FnOnce(&mut UiChildBuilder<AssetData>)
) -> Entity {
    image_panei(class, parent, (), children)
}

/// Spawns an [`ImageBundle`] with children.
pub fn image_panei<AssetData>(
    class: impl Class<AssetData, ImageBundle>,
    parent: &mut UiChildBuilder<AssetData>,
    extras: impl Bundle,
    children: impl FnOnce(&mut UiChildBuilder<AssetData>)
) -> Entity {
    let mut bundle = ImageBundle::default();
    class.apply(parent.asset_data, &mut bundle);
    parent
        .spawn((bundle, extras))
        .with_children(children).id()
}

/// Spawns a [`NodeBundle`] composed of [`NodeBundle`] cells in the form of a grid.
/// The callback function argument spawns the contents of those cells.
pub fn grid<AssetData>(
    rows: usize,
    columns: usize,
    class: impl Class<AssetData, NodeBundle>,
    parent: &mut UiChildBuilder<AssetData>,
    children: impl FnMut(&mut UiChildBuilder<AssetData>, usize, usize)
) -> Entity {
    gridi(rows, columns, class, (), parent, children)
}

/// Spawns a [`NodeBundle`] composed of [`NodeBundle`] cells in the form of a grid.
/// The callback function argument spawns the contents of those cells.
pub fn gridi<AssetData>(
    rows: usize,
    columns: usize,
    class: impl Class<AssetData, NodeBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder<AssetData>,
    mut children: impl FnMut(&mut UiChildBuilder<AssetData>, usize, usize)
) -> Entity {
    // Spawns container
    let mut container_bundle = NodeBundle::default();
    class.apply(parent.asset_data, &mut container_bundle);
    container_bundle.style.flex_wrap = FlexWrap::Wrap;
    let mut container = parent.spawn((container_bundle, extras));

    // Spawns cells as children of the container
    let cell_bundle = NodeBundle {
        style: Style {
            width: Val::Percent(100.0 / columns as f32),
            height: Val::Percent(100.0 / rows as f32),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    };
    for row in 0..rows {
        for col in 0..columns {
            container = container.with_children(|container| {
                container
                    .spawn(cell_bundle.clone())
                    .with_children(|cell| children(cell, row, col));
            });
        }
    }
    container.id()
}
