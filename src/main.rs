use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use kayak_ui::{widgets::KayakWidgets, prelude::KayakContextPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
        .add_plugin(EditorPlugin::default())
        .run();
}
