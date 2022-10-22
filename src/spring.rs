use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

use crate::particle::{Force, Position, Velocity};

#[derive(Component)]
struct Constants {
    ks: f32,
    kd: f32,
    r: f32,
}

#[derive(Component)]
struct Particles(Entity, Entity);

#[derive(Bundle)]
pub struct SpringBundle {
    constants: Constants,
    particles: Particles,
    #[bundle]
    shape: ShapeBundle,
}

impl SpringBundle {
    pub fn new(p1: Entity, p2: Entity, ks: f32, kd: f32, r: f32) -> Self {
        Self {
            constants: Constants { ks, kd, r },
            particles: Particles(p1, p2),
            shape: GeometryBuilder::build_as(
                &shapes::Line(Vec2::ZERO, Vec2::ZERO),
                DrawMode::Stroke(StrokeMode::new(Color::RED, 2.0)),
                Transform::default(),
            ),
        }
    }
}

fn compute(
    mut springs: Query<(&Particles, &Constants)>,
    mut particles: Query<(&Position, &Velocity, &mut Force)>,
) {
    for (Particles(p1, p2), Constants { ks, kd, r }) in springs.iter_mut() {
        if let Ok([(x1, v1, mut f1), (x2, v2, mut f2)]) = particles.get_many_mut([*p1, *p2]) {
            let dx = x1.0 - x2.0;
            let dv = v1.0 - v2.0;
            let f = (ks * (dx.length() - r) + kd * (dx.dot(dv) / dx.length())) * dx.normalize();
            f1.0 += -f;
            f2.0 += f;
        }
    }
}

fn sync_paths(mut springs: Query<(&Particles, &mut Path)>, particles: Query<&Position>) {
    for (Particles(p1, p2), mut path) in springs.iter_mut() {
        let pos1 = particles.get(*p1).unwrap();
        let pos2 = particles.get(*p2).unwrap();
        *path = ShapePath::build_as(&shapes::Line(pos1.0, pos2.0));
    }
}

pub struct SpringPlugin;

impl Plugin for SpringPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(compute).add_system(sync_paths);
    }
}
