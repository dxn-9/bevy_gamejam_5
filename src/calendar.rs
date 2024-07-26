use bevy::prelude::*;

use crate::{constants::DAY_DURATION, resources::Calendar};
pub fn update_calendar(
    mut r_calendar: ResMut<Calendar>,
    mut last_update: Local<f32>,
    r_time: Res<Time>,
) {
    let month = r_calendar.month;
    let days_in_month = month.days();

    if *last_update >= DAY_DURATION {
        if r_calendar.day == days_in_month {
            r_calendar.day = 1;
            r_calendar.month = month.next();
        } else {
            r_calendar.day += 1;
        }
        *last_update = 0.0;
    }

    *last_update += r_time.delta_seconds();
}
