mod constants;
mod world;

use std::f32::consts::PI;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::dev_tools::DevToolsPlugin;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::pbr::{CascadeShadowConfigBuilder, VolumetricFogSettings, VolumetricLight};
use bevy::prelude::*;

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
        );

    #[cfg(feature = "diagnostic")]
    app.add_plugins(
        FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    font: default(),
                },
            },
        },
    );

    app.init_state::<GameStatus>();

    // add systems
    app.add_systems(Startup, setup);

    app.run();
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
    commands
        .spawn(
            Camera3dBundle {
                transform: Transform::from_xyz(10.0, 12.0, 16.0)
                    .looking_at(Vec3::ZERO, Vec3::Y),
                camera: Camera {
                    hdr: false,
                    ..default()
                },
                tonemapping: Tonemapping::TonyMcMapface,
                ..default()
            }
        )
        .insert(constants::graphic_settings::DEFAULT_BLOOM_SETTINGS)
        .insert(VolumetricFogSettings { ambient_intensity: 0., ..default() });
    
    commands.spawn(constants::graphic_settings::DEFAULT_BLOOM_SETTINGS);

    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Cuboid::new(2., 2., 2.)),
            material: materials.add(Color::WHITE),
            ..default()
        }
    );

    // spawn the sun
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: light_consts::lux::OVERCAST_DAY,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            },
            ..default()
        },
        VolumetricLight
    ));

}