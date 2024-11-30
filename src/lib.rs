use bevy_app::App;
use bevy_app::Plugin;
#[cfg(not(target_family = "wasm"))]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_family = "wasm"))]
        app.add_plugins(WorldInspectorPlugin::new());
    }
}
