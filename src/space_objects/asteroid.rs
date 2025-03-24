use super::SpaceThingTrait;
use crate::Shape;
use bevy::{pbr::wireframe::NoWireframe, prelude::*};
use rand::Rng;
use std::f32::consts::PI;

pub const ASTEROID_MESH: &str = "mesh/debug.gltf";

#[derive(Clone, Debug, Component)]
pub struct Asteroid {
    /// the diameter of the asteroid in meters
    size: f32,
    /// end location
    going_to: Vec3,
    /// where to Spawn the asteroid.
    spawn_at: Vec3,
    /// speed in km/s
    speed: f32,
    /// the axis on which the object will rotate.
    rotation_axis: Vec2,
    rotation_speed: f32,
    travelled: f32,
    scale: f32,
    // location: Vec3,
}

impl Default for Asteroid {
    fn default() -> Self {
        let mut rng = rand::rng();
        // let half_pi = PI / 2.0;

        let size = rng.random_range(0.10..0.240);
        let speed = rng.random_range(0.165..0.250);
        // let speed = speed / 10.;

        // let theta_x = rng.random_range(-PI..PI);
        let theta = rng.random_range(0.0..2.0 * PI);
        let magnitude = 0.0;
        let spawn_at = (0.0, 0.0, magnitude).into();
        // info!("spawn_at => {}", spawn_at);
        // let theta_x = rng.random_range(0.0..PI);
        // let theta_y = rng.random_range(0.0..PI);
        let magnitude = size;
        let going_to = (theta, PI / 2.0, magnitude).into();
        // info!("going_to => {}", going_to);
        let rotation_axis = {
            let theta_x = rng.random_range(0.0..(2.0 * PI));
            let theta_y = rng.random_range(0.0..(2.0 * PI));
            (theta_x, theta_y).into()
        };
        let rotation_speed = rng.random_range(0.0..0.25);
        let travelled = 0.0;

        Self {
            size,
            speed,
            spawn_at,
            going_to,
            rotation_axis,
            rotation_speed,
            travelled,
            scale: 0.0,
            // location: Vec3::default(),
        }
    }
}

impl SpaceThingTrait for Asteroid {
    fn update_orientation(&mut self, time_delta: &Res<Time>, orientation: &mut Transform) {
        orientation.rotate_x(self.rotation_axis[0] * time_delta.delta_secs() * self.rotation_speed);
        orientation.rotate_y(self.rotation_axis[1] * time_delta.delta_secs() * self.rotation_speed);
    }

    fn update_location(&mut self, time_delta: &Res<Time>, location: &mut Transform) {
        // info!("{}", time_delta.delta_secs());
        let distance = self.speed * time_delta.delta_secs();
        self.travelled += distance;
        // let delta = if self.spawn_at.distance((0.0, 0.0, 8.0).into())
        //     > self.going_to.distance((0.0, 0.0, 8.0).into())
        // {
        //     self.spawn_at.lerp(self.going_to, self.travelled)
        // } else {
        //     self.going_to.lerp(self.spawn_at, self.travelled)
        // };
        let delta = self.spawn_at.lerp(self.going_to, self.travelled);
        // self.location = delta;

        location.translation = delta;
    }

    // fn get_mesh(&self) -> impl Into<PathBuf> {
    //     ASTEROID_MESH
    // }
    fn spawn_model<'a>(
        &mut self,
        // mut cmds: Commands,
        asset_server: &Res<AssetServer>,
        // meshes: Res<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        // transform: Transform,
        fov: f32,
    ) -> impl Bundle {
        // Load the mesh and texture
        // _ = asset_server.load_folder("models/asteroid/");
        let mesh_palette = asset_server.load("models/asteroid/texture.png");
        let mesh_handle = asset_server.load(
            GltfAssetLabel::Primitive {
                mesh: 0,
                primitive: 0,
            }
            .from_asset("models/asteroid/model.gltf"),
        );

        // Create a material
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(mesh_palette.clone()),
            ..Default::default()
        });
        let transform = self.get_transform(fov);

        (
            // self.clone(),
            // Mesh3d(cube),
            Mesh3d(mesh_handle),
            // MeshMaterial3d(debug_material.clone()),
            // MeshMaterial3d(debug_material.0.clone()),
            MeshMaterial3d(material_handle),
            // Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(rot_1 * rot_2),
            transform,
            NoWireframe,
            Shape,
            // Visibility::Visible,
        )
    }

    fn get_transform(&mut self, fov: f32) -> Transform {
        // let fov = 10000.0;
        // let mut tmp_loc = self.spawn_at.clone();
        // tmp_loc[2] = fov;
        // self.spawn_at[0] = tmp_loc[2] * tmp_loc[1].sin() * tmp_loc[0].cos();
        // self.spawn_at[1] = tmp_loc[2] * tmp_loc[1].sin() * tmp_loc[0].sin();
        // self.spawn_at[2] = tmp_loc[2] * tmp_loc[1].cos();
        self.spawn_at[0] = 0.0;
        self.spawn_at[1] = 0.0;
        self.spawn_at[2] = -fov;

        self.speed = (self.spawn_at.distance(self.going_to) * self.speed) / fov;

        // let scale = (1.0 / fov) * self.size;
        let scale = self.size * (fov * 0.05);
        // let scale = self.size / fov;
        // let scale = self.size;
        self.scale = scale;
        // info!("scale = {scale}");
        // info!("size = {}", self.size);
        self.going_to[2] *= fov * 0.5 + scale;

        // self.speed /= scale;

        {
            let tmp_loc = self.going_to.clone();
            self.going_to[0] = tmp_loc[2] * tmp_loc[1].sin() * tmp_loc[0].cos();
            self.going_to[1] = tmp_loc[2] * tmp_loc[1].sin() * tmp_loc[0].sin();
            self.going_to[2] = tmp_loc[2] * tmp_loc[1].cos();
        }

        // self.going_to = Vec3::ZERO;

        // self.speed /= fov;

        // Transform::from_xyz(self.spawn_at[0], self.spawn_at[1], self.spawn_at[2])
        Transform::from_xyz(self.spawn_at[0], self.spawn_at[1], self.spawn_at[2])
            .with_scale(Vec3::new(scale, scale, scale))
    }

    // fn should_despawn(&self) -> bool {
    //     info!("{}", self.location[2]);
    //     self.location[2] >= 8.0 + self.size
    // }
}
