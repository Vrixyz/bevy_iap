use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_iap::GamePlugin;

#[bevy_main]
fn main() {
    dbg!("what");
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::INFO,
                    filter: "wgpu=warn,bevy_ecs=info,winit=error".to_string(),
                    update_subscriber: None,
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            GamePlugin,
        ))
        .run()
}
