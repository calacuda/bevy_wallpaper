use bevy::{
    a11y::AccessibilityPlugin,
    asset::RenderAssetUsages,
    audio::AudioPlugin,
    log::{Level, LogPlugin},
    pbr::wireframe::{NoWireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        pipelined_rendering::PipelinedRenderingPlugin,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    window::{
        PresentMode, WindowCreated, WindowLevel, WindowMode, WindowResized, WindowResolution,
    },
    winit::{WakeUp, WinitPlugin},
};
use bevy_linux_wallpaper::WallpaperPlugin;
use bevy_wallpaper::{
    Shape,
    space_objects::{SpaceThing, SpaceThingTrait, asteroid::Asteroid},
};
use std::f32::consts::PI;

#[derive(Component)]
struct DebugTexture(pub Handle<StandardMaterial>);

fn main() {
    let mut wp_plug = WallpaperPlugin::<WakeUp>::default();
    wp_plug.run_on_any_thread = true;

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::INFO,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoVsync,
                        name: Some("wallpaper".into()),
                        window_level: WindowLevel::AlwaysOnBottom,
                        mode: WindowMode::Windowed,
                        skip_taskbar: false,
                        titlebar_shown: false,
                        resolution: WindowResolution::new(1920., 1080.),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .disable::<PipelinedRenderingPlugin>()
                .disable::<AccessibilityPlugin>()
                .disable::<AudioPlugin>()
                .disable::<WinitPlugin>(),
            WireframePlugin,
            // ObjPlugin,
            wp_plug,
        ))
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: true,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: Srgba {
                red: (166. / 255.),
                green: (227. / 255.),
                blue: (161. / 255.),
                alpha: 1.0,
            }
            .into(),
        })
        // .add_systems(Startup, (camera_setup, spawn_spacething).chain())
        .add_systems(Startup, camera_setup)
        .add_systems(
            Update,
            (
                mod_spacething_transform,
                log_window_resize,
                window_creation_log,
                log_window_move,
                despawn_spacethings,
                spawn_spacething.run_if(time_to_spawn),
                // log_assets,
            ),
        )
        .run();
}

fn camera_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    commands.insert_resource(ClearColor(
        Srgba {
            red: (30. / 255.),
            green: (30. / 255.),
            blue: (46. / 255.),
            alpha: 1.0,
        }
        .into(),
    ));
    // commands.insert_resource(ClearColor(
    //     Srgba {
    //         red: 0.,
    //      green: 0.,
    //         blue: 0.,
    //         alpha: 1.0,
    //     }
    //     .into(),
    // ));

    commands.spawn(DebugTexture(debug_material));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        Camera::default(),
        Projection::Perspective(PerspectiveProjection {
            // far: 1_000.0,
            far: 1_000_000.0,
            ..default()
        }),
    ));

    let intensity = 10_000_000.0;
    let light = PointLight {
        shadows_enabled: true,
        // intensity: 10_000_000.,
        intensity,
        range: 10_000_000.0,
        shadow_depth_bias: 0.2,
        radius: PI * 0.5,
        ..default()
    };

    commands.spawn((
        light,
        // Transform::from_xyz(8.0, 16.0, 8.0),
        Transform::from_xyz(1.0, 1.0, 8.0).looking_at(Vec3::new(1.0, 1.0, 0.0), Vec3::Y),
    ));

    commands.spawn((
        light,
        // Transform::from_xyz(8.0, 16.0, 8.0),
        Transform::from_xyz(-1.0, 1.0, 8.0).looking_at(Vec3::new(-1.0, 1.0, 0.0), Vec3::Y),
    ));

    commands.spawn((
        light,
        // Transform::from_xyz(8.0, 16.0, 8.0),
        Transform::from_xyz(1.0, -1.0, 8.0).looking_at(Vec3::new(1.0, -1.0, 0.0), Vec3::Y),
    ));

    commands.spawn((
        light,
        // Transform::from_xyz(8.0, 16.0, 8.0),
        Transform::from_xyz(-1.0, -1.0, 8.0).looking_at(Vec3::new(-1.0, -1.0, 0.0), Vec3::Y),
    ));
}

fn spawn_spacething(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // debug_material: Single<&DebugTexture>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // let sphere = meshes.add(Sphere::default());

    let mut space_thing = SpaceThing::Asteroid(Asteroid::default());

    commands
        .spawn(space_thing.spawn_model(&asset_server, &mut materials, 1_000_000.0))
        .insert(space_thing);

    // info!("spawning spacething");
    debug!("spawning spacething");
}

fn mod_spacething_transform(
    mut query: Query<(&mut SpaceThing, &mut Transform), With<Shape>>,
    time: Res<Time>,
) {
    for (mut space_thing, mut transform) in &mut query {
        space_thing.update_orientation(&time, &mut transform);
        space_thing.update_location(&time, &mut transform);
        // info!("space thing location = {}", transform.translation);
        debug!("space thing location = {}", transform.translation);
    }
}

fn log_window_resize(mut resize_reader: EventReader<WindowResized>) {
    for e in resize_reader.read() {
        // When resolution is being changed
        info!("size {:.1} x {:.1}", e.width, e.height);
    }
}

fn log_window_move(mut resize_reader: EventReader<WindowMoved>) {
    for e in resize_reader.read() {
        // When resolution is being changed
        info!("location {} x {}", e.position[0], e.position[1]);
    }
}

fn window_creation_log(mut created_evs: EventReader<WindowCreated>) {
    for e in created_evs.read() {
        info!("window created{e:?}");
    }
}

fn despawn_spacethings(mut cmds: Commands, space_things: Query<(&SpaceThing, &Transform, Entity)>) {
    for (space_thing, transform, entity) in space_things.iter() {
        if space_thing.should_despawn() || transform.translation[2] > 8.0 {
            cmds.entity(entity).despawn_recursive();
            // info!("despawning spacething");
            debug!("despawning spacething");
        }
    }
}

fn time_to_spawn(time: Res<Time>) -> bool {
    // info!("{}", time.elapsed_secs() % 3.0);
    time.elapsed_secs() % 2.5 <= 0.0303
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

// fn log_assets(mut asset_evs: EventReader<AssetEvent<Mesh>>) {
//     for ev in asset_evs.read() {
//         warn!("{ev:?}");
//         // match ev.to_owned() {
//         //     AssetEvent::Added { id: _ } => {
//         //         info!("Added");
//         //     }
//         //     _ => {}
//         // }
//     }
// }
