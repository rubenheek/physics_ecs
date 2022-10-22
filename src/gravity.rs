use bevy::prelude::Query;

use crate::particle::{Force, Mass, Position};

pub fn linear_gravity(mut query: Query<(&mut Force, &Mass)>) {
    let g = 9.81;
    for (mut force, mass) in query.iter_mut() {
        force.0.y -= mass.0 * g;
    }
}

fn _prod_gravity(mut query: Query<(&mut Force, &Position, &Mass)>) {
    let g = 1.;
    let mut combinatins = query.iter_combinations_mut();
    while let Some([(mut f1, pos1, m1), (mut f2, pos2, m2)]) = combinatins.fetch_next() {
        let r = pos1.0 - pos2.0;
        let f = m1.0 * m2.0 * g / r.dot(r);
        f1.0 += f;
        f2.0 += f;
    }
}
