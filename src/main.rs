use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use physics_ecs::{
    // gravity::linear_gravity,
    particle::{Mass, ParticleBundle, ParticlePlugin, Position},
    solver::newton,
    spring::{SpringBundle, SpringPlugin},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(ParticlePlugin)
        .add_plugin(SpringPlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn)
        // .add_system(linear_gravity)
        .add_system(newton)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn(mut commands: Commands) {
    let p1 = commands
        .spawn_bundle(ParticleBundle {
            position: Position(Vec2::new(0., 0.)),
            mass: Mass(1.),
            ..default()
        })
        .id();
    let p2 = commands
        .spawn_bundle(ParticleBundle {
            position: Position(Vec2::new(100., 0.)),
            mass: Mass(1.),
            ..default()
        })
        .id();
    commands.spawn_bundle(SpringBundle::new(p1, p2, 1., 1., 10.0));
}
