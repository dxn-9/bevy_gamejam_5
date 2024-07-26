use bevy::prelude::*;

#[derive(Default, Component, Clone, Copy, Debug, PartialEq)]
pub struct Ground;

#[derive(Default, Component, Clone, Copy, Debug, PartialEq)]
pub struct PlayerMesh;
#[derive(Default, Component, Clone, Copy, Debug, PartialEq)]
pub struct Player;
#[derive(Default, Component, Clone, Copy, Debug, PartialEq)]
pub enum CameraMode {
    #[default]
    Follow,
    Free,
}

#[derive(Default, Component, Clone, Copy, Debug, PartialEq)]
pub enum Seasons {
    #[default]
    Spring,
    Summer,
    Autumn,
    Winter,
}
#[derive(Default, Component, Clone, Copy, Debug, PartialEq)]
pub struct Earth;

#[derive(Default, Component, Clone, Copy, Debug, PartialEq)]
pub struct Velocity(Vec3);
#[derive(Default, Component, Clone, Copy, Debug, Deref, DerefMut)]
pub struct FreeModeTransform(Transform);

#[derive(Debug, Component)]
pub struct PlayerCamera;
#[derive(Debug, Component)]
pub struct FreeCamera;
