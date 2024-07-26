use bevy::prelude::*;

pub struct ResourcesPlugin;

#[derive(Default, Resource, Clone, Copy, Debug, PartialEq)]
pub enum Months {
    #[default]
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
impl Months {
    pub fn days(&self) -> u8 {
        match self {
            Months::January => 31,
            Months::February => 28,
            Months::March => 31,
            Months::April => 30,
            Months::May => 31,
            Months::June => 30,
            Months::July => 31,
            Months::August => 31,
            Months::September => 30,
            Months::October => 31,
            Months::November => 30,
            Months::December => 31,
        }
    }
    pub fn next(&self) -> Months {
        match self {
            Months::January => Months::February,
            Months::February => Months::March,
            Months::March => Months::April,
            Months::April => Months::May,
            Months::May => Months::June,
            Months::June => Months::July,
            Months::July => Months::August,
            Months::August => Months::September,
            Months::September => Months::October,
            Months::October => Months::November,
            Months::November => Months::December,
            Months::December => Months::January,
        }
    }
}

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct Calendar {
    pub month: Months,
    pub day: u8,
}
impl Default for Calendar {
    fn default() -> Self {
        Calendar {
            month: Months::January,
            day: 1,
        }
    }
}

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
