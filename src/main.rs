mod constants;
mod world;

use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
#[cfg(feature = "diagnostic")]
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: constants::DEFAULT_PRESENT_MODE,
                    mode: constants::DEFAULT_WINDOW_MODE,
                    title: constants::WINDOW_NAME.to_string(),
                    name: Some(constants::APP_ID.to_string()),
                    resizable: true,
                    decorations: true,
                    transparent: false,
                    focused: true,
                    prevent_default_event_handling: true,
                    internal: Default::default(),
                    visible: true,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(bevy_flycam::prelude::PlayerPlugin);
    
    #[cfg(feature = "diagnostic")]
    app.add_plugins((ScreenDiagnosticsPlugin::default(), ScreenFrameDiagnosticsPlugin));

    app.init_state::<GameStatus>();

    // add systems
    app.add_systems(Startup, setup);

    app.run()
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash, Debug, States)]
enum GameStatus {
    MainMenu,
    #[default]
    InGame,
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    // commands.spawn((
    //     Camera3dBundle {
    //         transform: Transform::from_xyz(10.0, 12.0, 16.0)
    //             .looking_at(Vec3::ZERO, Vec3::Y),
    //         camera: Camera {
    //             hdr: false,
    //             ..default()
    //         },
    //         tonemapping: Tonemapping::TonyMcMapface,
    //         ..default()
    //     },
    //     constants::graphic_settings::DEFAULT_BLOOM_SETTINGS
    // ));
    commands.spawn(constants::graphic_settings::DEFAULT_BLOOM_SETTINGS);

    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Cuboid::new(2., 2., 2.)),
            material: materials.add(Color::WHITE),
            ..default()
        }
    );
}