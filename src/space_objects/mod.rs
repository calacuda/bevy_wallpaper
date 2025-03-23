use asteroid::Asteroid;
use bevy::prelude::*;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

pub mod asteroid;

#[enum_dispatch(SpaceThing)]
pub trait SpaceThingTrait: Default {
    /// updates the 3D rotation of the SpaceThing
    fn update_orientation(&mut self, time_delta: &Res<Time>, orientation: &mut Transform);
    /// updates the 3D location of the SpaceThing
    fn update_location(&mut self, time_delta: &Res<Time>, location: &mut Transform);
    /// returns the asset path to the mesh
    fn get_mesh(&self) -> impl Into<PathBuf>;
    /// set FOV in km;
    fn get_transform(&mut self, fov: f32) -> Transform;
    /// will return true when the SpaceThing is out of view and should despawn.
    fn should_despawn(&self) -> bool {
        false
    }
}

#[enum_dispatch]
#[derive(Clone, Debug, Component)]
pub enum SpaceThing {
    Asteroid(Asteroid),
    // SpacePerson,
    // Alian,
    // AlianShip,
    // SatiLite,
}

impl Default for SpaceThing {
    fn default() -> Self {
        Self::Asteroid(Asteroid::default())
    }
}
