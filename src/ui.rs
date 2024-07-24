use bevy::prelude::*;

use crate::resources::Belief;

#[derive(Component, Debug)]
pub struct BeliefBar;

pub fn setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect {
                    top: Val::Px(10.0),
                    ..Default::default()
                },
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|commands| {
            commands.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(20.0),
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    background_color: Color::linear_rgb(1.0, 0.0, 0.0).into(),
                    ..Default::default()
                },
                BeliefBar,
            ));
        });
}

pub fn update_ui(mut q_belief_bar: Query<&mut Style, With<BeliefBar>>, belief: Res<Belief>) {
    let mut belief_bar = q_belief_bar.single_mut();
    belief_bar.width = Val::Percent(belief.value * 100.0);
}
