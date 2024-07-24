use bevy::prelude::*;

pub struct ResourcesPlugin;

#[derive(Default, Resource, Clone, Copy, Debug, PartialEq, Deref, DerefMut)]
pub struct MovementInput(Vec3);

#[derive(Default, Resource)]
pub struct Belief {
    pub value: f32,
}

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Belief>()
            .init_resource::<MovementInput>();
    }
}
