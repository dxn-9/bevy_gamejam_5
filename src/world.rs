use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{BLACK, BROWN, GRAY, RED, WHITE, YELLOW},
    math::{vec2, vec3},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::{
    components::{Earth, Ground, Player, PlayerCamera, PlayerMesh, Seasons},
    constants::{CAMERA_DISTANCE_Y, CAMERA_DISTANCE_Z},
};

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sun
    let sun_radius = 4.0;
    let sun_position = vec3(0.0, 0.0, -20.0);

    commands
        .spawn(TransformBundle::from_transform(
            Transform::from_translation(sun_position),
        ))
        .with_children(|c| {
            c.spawn(PointLightBundle {
                transform: Transform::from_translation(Vec3::Y),
                point_light: PointLight {
                    intensity: 100_000.0,
                    range: 100.0,
                    ..Default::default()
                },
                ..Default::default()
            });

            c.spawn((
                PbrBundle {
                    mesh: meshes.add(Sphere::new(sun_radius)),
                    material: materials.add(StandardMaterial {
                        emissive: YELLOW.into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                RigidBody::Fixed,
                Collider::ball(sun_radius),
            ));
        });

    // Earth
    {
        let earth_radius = 1.4;
        let earth_position = vec3(0.0, earth_radius, -24.0);
        commands
            .spawn(TransformBundle::from_transform(
                Transform::from_translation(earth_position),
            ))
            .with_children(|c| {
                c.spawn((
                    PbrBundle {
                        mesh: meshes.add(Sphere::new(earth_radius)),
                        material: materials.add(StandardMaterial {
                            base_color: BROWN.into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Earth,
                    RigidBody::Dynamic,
                    Collider::ball(earth_radius),
                    CollisionGroups::new(Group::GROUP_2, Group::ALL),
                ));
            });

        // Earth cage
        let cage_radius = 15.0;
        let pieces = 8;

        const SEASONS: [Seasons; 4] = [
            Seasons::Spring,
            Seasons::Summer,
            Seasons::Autumn,
            Seasons::Winter,
        ];
        commands
            .spawn(TransformBundle::from_transform(
                Transform::from_translation(sun_position),
            ))
            .with_children(|c| {
                for i in 0..pieces {
                    let angle = (2. * PI / pieces as f32) * i as f32;
                    c.spawn((
                        Collider::cuboid(cage_radius / 2., 5., 1.),
                        RigidBody::Fixed,
                        CollisionGroups::new(Group::GROUP_1, Group::GROUP_2),
                        TransformBundle::from_transform(
                            Transform::from_xyz(
                                cage_radius * angle.sin(),
                                0.0,
                                cage_radius * angle.cos(),
                            )
                            .with_rotation(Quat::from_rotation_y(angle)),
                        ),
                    ));
                }
                let season_position = [
                    vec2(0.5, 0.5),
                    vec2(0.5, -0.5),
                    vec2(-0.5, 0.5),
                    vec2(-0.5, -0.5),
                ];
                for (season, pos) in SEASONS.iter().zip(season_position.iter()) {
                    c.spawn((
                        Collider::cuboid(cage_radius / 2., 1., cage_radius / 2.),
                        season.clone(),
                        Sensor,
                        TransformBundle::from_transform(Transform::from_xyz(
                            cage_radius * pos.x,
                            0.,
                            cage_radius * pos.y,
                        )),
                    ));
                }
            });

        // Seasons sensor
    }

    // Platform
    commands.spawn((
        RigidBody::Fixed,
        Collider::cylinder(0.1, 50.0),
        PbrBundle {
            mesh: meshes.add(Cylinder::new(50.0, 0.2)),
            material: materials.add(StandardMaterial {
                base_color: GRAY.into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        Ground,
        Name::new("Ground"),
    ));

    // Player

    // https://github.com/dimforge/rapier/issues/497
    let player_collision_groups = CollisionGroups::new(Group::GROUP_1, Group::ALL);
    commands
        .spawn((
            RigidBody::KinematicPositionBased,
            KinematicCharacterController {
                custom_mass: Some(5.0),
                offset: CharacterLength::Absolute(0.1),
                slide: true,
                snap_to_ground: None,
                filter_groups: Some(player_collision_groups),

                ..Default::default()
            },
            player_collision_groups,
            TransformBundle::from_transform(Transform::from_xyz(0.0, 20.0, 0.0)),
            Collider::capsule_y(1.7 / 2.0, 0.8),
            VisibilityBundle::default(),
            Velocity::default(),
            Player,
            Name::new("Player"),
        ))
        .with_children(|c| {
            c.spawn((
                PbrBundle {
                    mesh: meshes.add(Capsule3d::new(0.8, 1.7)),
                    material: materials.add(StandardMaterial {
                        base_color: RED.into(),
                        emissive: BLACK.into(),
                        ..Default::default()
                    }),

                    ..Default::default()
                },
                PlayerMesh,
            ))
            .with_children(|c| {
                c.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.8, 0.4, 0.2)),
                    transform: Transform::from_xyz(0.0, 1.0, -0.8),
                    material: materials.add(Color::srgb_u8(0, 0, 200)),
                    ..Default::default()
                });
            });
            c.spawn((
                Camera3dBundle {
                    transform: Transform::from_xyz(0.0, CAMERA_DISTANCE_Y, CAMERA_DISTANCE_Z)
                        .looking_at(Vec3::ZERO, Dir3::Y),
                    camera: Camera {
                        is_active: false,
                        ..Default::default()
                    },
                    ..default()
                },
                Player,
                Name::new("Camera"),
                PlayerCamera,
            ));
        });
}

pub fn check_seasons_for_world(
    rapier_context: Res<RapierContext>,
    q_planet: Query<Entity, With<Earth>>,
    q_seasons: Query<(Entity, &Seasons)>,
) {
    let planet = q_planet.single();
    for (entity, season) in q_seasons.iter() {
        if rapier_context.intersection_pair(entity, planet) == Some(true) {
            println!("The planet is in the {:?} season!", season);
        }
    }
}
