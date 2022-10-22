use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

#[derive(Component, Default)]
pub struct Position(pub Vec2);

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct Force(pub Vec2);

#[derive(Component, Default)]
pub struct Mass(pub f32);

#[derive(Bundle)]
pub struct ParticleBundle {
    pub mass: Mass,
    pub position: Position,
    pub velocity: Velocity,
    pub force: Force,
    #[bundle]
    pub shape: ShapeBundle,
}

impl Default for ParticleBundle {
    fn default() -> Self {
        Self {
            mass: Default::default(),
            position: Default::default(),
            velocity: Default::default(),
            force: Default::default(),
            shape: GeometryBuilder::build_as(
                &shapes::Circle {
                    radius: 5.0,
                    center: Vec2::ZERO,
                },
                DrawMode::Fill(FillMode::color(Color::ORANGE)),
                Transform::default(),
            ),
        }
    }
}

fn sync_transforms(mut query: Query<(&mut Transform, &Position)>) {
    for (mut t, pos) in query.iter_mut() {
        t.translation.x = pos.0.x;
        t.translation.y = pos.0.y;
    }
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sync_transforms);
    }
}
