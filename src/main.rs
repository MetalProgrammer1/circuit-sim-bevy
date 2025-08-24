use avian2d::prelude::*;
use bevy::text::FontSmoothing;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

use bevy::{prelude::*, time::common_conditions::on_timer};
use std::{collections::HashSet, time::Duration};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (set_up, set_up_sensors))
        .add_systems(
            Update,
            spawn_electrons.run_if(on_timer(Duration::from_secs(2))),
        )
        .add_systems(Update, (move_electrons, change_direction))
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    // Here we define size of our overlay
                    font_size: 42.0,
                    // If we want, we can use a custom font
                    font: default(),
                    // We could also disable font smoothing,
                    font_smoothing: FontSmoothing::default(),
                    ..default()
                },
                // We can also change color of the overlay
                text_color: Color::srgb(0.0, 1.0, 0.0),
                // We can also set the refresh interval for the FPS counter
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
            },
        })
        .run();
}

#[derive(Resource)]
struct NumOfElectrons {
    num: i32,
}

#[derive(Component)]
struct Direction(f32, f32, f32, f32);

#[derive(Component)]
struct Electron;

#[derive(Component)]
struct NumCollisions(i32);

fn set_up(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape1 = meshes.add(Rectangle::new(10., 300.));
    let shape2 = meshes.add(Rectangle::new(10., 200.));
    let shape3 = meshes.add(Rectangle::new(400., 10.));
    let shape4 = meshes.add(Rectangle::new(300., 10.));
    let shape5 = meshes.add(Rectangle::new(185., 10.));
    let shape6 = meshes.add(Rectangle::new(135., 10.));
    let shape7 = meshes.add(Rectangle::new(10., 150.));
    let shape8 = meshes.add(Rectangle::new(10., 100.));
    let color = materials.add(Color::srgb(1., 0., 0.));
    commands.insert_resource(NumOfElectrons { num: 0 });
    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(shape1.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(-200., 0., 0.),
    ));
    commands.spawn((
        Mesh2d(shape2.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(-150., 0., 0.),
    ));
    commands.spawn((
        Mesh2d(shape1),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(200., 0., 0.),
    ));
    commands.spawn((
        Mesh2d(shape2),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(150., 0., 0.),
    ));

    commands.spawn((
        Mesh2d(shape3.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(0., -150., 0.),
    ));
    //change this
    commands.spawn((
        Mesh2d(shape5.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(-110., 150., 0.),
    ));
    commands.spawn((
        Mesh2d(shape5),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(110., 150., 0.),
    ));

    commands.spawn((
        Mesh2d(shape4.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(0., -100., 0.),
    ));
    commands.spawn((
        Mesh2d(shape6.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(85., 100., 0.),
    ));
    commands.spawn((
        Mesh2d(shape6.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(-85., 100., 0.),
    ));
    commands.spawn((
        Mesh2d(shape7.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(-15., 125., 0.),
    ));
    commands.spawn((
        Mesh2d(shape8.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(15., 125., 0.),
    ));
}

fn set_up_sensors(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(40., 245.))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
        Transform::from_xyz(-175., 125., -1.),
        Collider::rectangle(1., 1.),
        Sensor,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(170., 40.))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
        Transform::from_xyz(175., 125., -1.),
        Collider::rectangle(1., 1.),
        Sensor,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(345., 40.))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
        Transform::from_xyz(-175., -125., -1.),
        Collider::rectangle(1., 1.),
        Sensor,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(40., 245.))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
        Transform::from_xyz(175., -125., -1.),
        Collider::rectangle(1., 1.),
        Sensor,
    ));
}

fn spawn_electrons(
    mut num_of_electrons: ResMut<NumOfElectrons>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if num_of_electrons.num < 20 {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(15.))),
            Collider::circle(1.),
            MeshMaterial2d(materials.add(Color::srgb(0., 1., 0.))),
            Transform::from_xyz(0., 125., 0.),
            Direction(1., 0., 0., 0.),
            NumCollisions(0),
            Electron,
            Sensor,
            CollisionEventsEnabled,
        ));
    }
    num_of_electrons.num += 1;
}

fn move_electrons(mut query: Query<(&mut Transform, &Direction), With<Electron>>, time: Res<Time>) {
    for (mut transform, direction) in &mut query {
        transform.translation.x += -30.0 * time.delta_secs() * direction.0;
        transform.translation.y += -30.0 * time.delta_secs() * direction.1;
        transform.translation.x += 30.0 * time.delta_secs() * direction.2;
        transform.translation.y += 30.0 * time.delta_secs() * direction.3;
    }
}

fn change_direction(
    mut electron_query: Query<(Entity, &mut NumCollisions, &mut Direction), With<Electron>>,
    mut collision_event_reader: EventReader<CollisionStarted>,
) {
    let mut collided_electrons = HashSet::new();
    for CollisionStarted(e1, e2) in collision_event_reader.read() {
        collided_electrons.insert(*e1);
        collided_electrons.insert(*e2);
    }
    for (electron, mut numcollisions, mut direction) in &mut electron_query {
        if collided_electrons.contains(&electron) {
            numcollisions.0 += 1;
            if numcollisions.0 == 0 || numcollisions.0 % 4 == 0 {
                direction.0 = 1.0;
                direction.1 = 0.0;
                direction.2 = 0.0;
                direction.3 = 0.0;
            } else if numcollisions.0 - 1 == 0 || (numcollisions.0 - 1) % 4 == 0 {
                direction.0 = 0.0;
                direction.1 = 1.0;
                direction.2 = 0.0;
                direction.3 = 0.0;
            } else if numcollisions.0 - 2 == 0 || (numcollisions.0 - 2) % 4 == 0 {
                direction.0 = 0.0;
                direction.1 = 0.0;
                direction.2 = 1.0;
                direction.3 = 0.0;
            } else if numcollisions.0 - 3 == 0 || (numcollisions.0 - 3) % 4 == 0 {
                direction.0 = 0.0;
                direction.1 = 0.0;
                direction.2 = 0.0;
                direction.3 = 1.0;
            }
        }
    }
}
