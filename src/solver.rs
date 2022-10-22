use bevy::prelude::*;

use crate::particle::{Force, Mass, Position, Velocity};

pub fn newton(
    time: Res<Time>,
    mut query: Query<(&mut Position, &mut Velocity, &mut Force, &Mass)>,
) {
    let dt = time.delta_seconds();
    for (mut position, mut velocity, mut force, mass) in query.iter_mut() {
        position.0 += velocity.0 * dt;
        velocity.0 += force.0 / mass.0 * dt;
        force.0 = Vec2::ZERO;
    }
}
