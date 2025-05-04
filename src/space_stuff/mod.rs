use crate::{Mode, Shape};
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use space_objects::{SpaceThing, SpaceThingTrait, asteroid::Asteroid, astronaut::Astronaut};
use std::f32::consts::PI;

pub mod space_objects;

#[derive(Default)]
pub struct SpaceStuff;

impl Plugin for SpaceStuff {
    fn build(&self, app: &mut App) {
        // let mut wp_plug = WallpaperPlugin::<WakeUp>::default();
        // wp_plug.run_on_any_thread = true;

        // App::new()
        app.add_systems(OnEnter(Mode::SpaceStuff), camera_setup)
            .add_systems(OnExit(Mode::SpaceStuff), camera_teardown)
            .add_systems(
                Update,
                (
                    mod_spacething_transform,
                    despawn_spacethings,
                    spawn_spacething.run_if(time_to_spawn),
                    // log_assets,
                )
                    .run_if(in_state(Mode::SpaceStuff)),
            );
    }
}

fn camera_setup(
    mut commands: Commands,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    // mut images: ResMut<Assets<Image>>,
) {
    // let debug_material = materials.add(StandardMaterial {
    //     base_color_texture: Some(images.add(uv_debug_texture())),
    //     ..default()
    // });

    // commands.insert_resource(ClearColor(
    //     Srgba {
    //         red: (30. / 255.),
    //         green: (30. / 255.),
    //         blue: (46. / 255.),
    //         alpha: 1.0,
    //     }
    //     .into(),
    // ));
    // commands.insert_resource(ClearColor(
    //     Srgba {
    //         red: 0.,
    //      green: 0.,
    //         blue: 0.,
    //         alpha: 1.0,
    //     }
    //     .into(),
    // ));

    // commands.spawn(DebugTexture(debug_material));

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
        range: 1_000.0,
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

fn camera_teardown(
    mut cmds: Commands,
    mut cameras: Query<Entity, With<Camera>>,
    mut lights: Query<Entity, With<PointLight>>,
) {
    for camera in cameras.iter() {
        cmds.entity(camera).despawn_recursive()
    }

    for light in lights.iter() {
        cmds.entity(light).despawn_recursive()
    }
}

fn spawn_spacething(
    cmds: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // debug_material: Single<&DebugTexture>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // let sphere = meshes.add(Sphere::default());

    // let mut space_thing = SpaceThing::Asteroid(Asteroid::default());
    let mut space_thing = SpaceThing::Astronaut(Astronaut::default());

    // commands
    //     .spawn(space_thing.spawn_model(&asset_server, &mut materials, 1_000_000.0))
    //     .insert(space_thing);
    // let id = cmds.spawn(space_thing).id();
    space_thing.spawn_model(cmds, &asset_server, &mut materials, 1_000_000.0);

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

// /// Creates a colorful test pattern
// fn uv_debug_texture() -> Image {
//     const TEXTURE_SIZE: usize = 8;
//
//     let mut palette: [u8; 32] = [
//         255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
//         198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
//     ];
//
//     let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
//     for y in 0..TEXTURE_SIZE {
//         let offset = TEXTURE_SIZE * y * 4;
//         texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
//         palette.rotate_right(4);
//     }
//
//     Image::new_fill(
//         Extent3d {
//             width: TEXTURE_SIZE as u32,
//             height: TEXTURE_SIZE as u32,
//             depth_or_array_layers: 1,
//         },
//         TextureDimension::D2,
//         &texture_data,
//         TextureFormat::Rgba8UnormSrgb,
//         RenderAssetUsages::RENDER_WORLD,
//     )
// }

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
