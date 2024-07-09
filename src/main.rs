mod constants;
mod world;
mod flycam;

use std::f32::consts::PI;
use bevy::core_pipeline::fxaa::Fxaa;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::pbr::{VolumetricFogSettings, VolumetricLight};
use bevy::prelude::*;
use bevy::render::view::{GpuCulling, NoCpuCulling};
use crate::flycam::FlyCam;
use crate::world::WorldPlugin;

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
        .add_plugins(flycam::NoCameraPlayerPlugin);

    #[cfg(feature = "diagnostic")]
    {
        use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
        
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
    }

    app.init_state::<GameStatus>();

    app.add_plugins(WorldPlugin);

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
    mut commands: Commands
){
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(10.0, 12.0, 16.0)
                    .looking_at(Vec3::ZERO, Vec3::Y),
                camera: Camera {
                    hdr: false,
                    ..default()
                },
                tonemapping: Tonemapping::TonyMcMapface,
                ..default()
            },
            FlyCam
        ))
        .insert(constants::graphic_settings::DEFAULT_BLOOM_SETTINGS)
        .insert(VolumetricFogSettings { ambient_intensity: 0.5, absorption: 0.15, density: 0.02, ..default() });

    commands.spawn(constants::graphic_settings::DEFAULT_BLOOM_SETTINGS);

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
        VolumetricLight,
        Fxaa::default(),
        // Enable the GPU Frustum culling and disable the cpu culling
        GpuCulling,
        NoCpuCulling
    ));

    // Insert ambient light
    commands.insert_resource(
        AmbientLight {
            color: Default::default(),
            brightness: light_consts::lux::AMBIENT_DAYLIGHT,
        }
    );
}