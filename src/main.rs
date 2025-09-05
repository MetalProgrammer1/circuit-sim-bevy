use avian2d::prelude::*;
use bevy::text::FontSmoothing;

use bevy::window::PrimaryWindow;
use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use std::{collections::HashSet, time::Duration};
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (500., 500.).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        //.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (set_up, set_up_sensors))
        .add_systems(
            Update,
            spawn_electrons.run_if(on_timer(Duration::from_secs(1))),
        )
        .add_systems(
            Update,
            (move_electrons, change_direction_or_speed, spawn_resistors),
        )
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
struct Direction(f32, f32, f32, f32, f32);

#[derive(Component)]
struct Electron;

#[derive(Component)]
struct NumCollisions(i32);
/*
#[derive(Resource, default)]
struct Coordinates(Vec2);
*/
#[derive(Component)]
struct Resistor;

#[derive(Component)]
struct BeginResistor(i32);

#[derive(Component)]
struct EndResistor(i32);
#[derive(Component)]
struct MainCamera;

fn set_up(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(NumOfElectrons { num: 0 });
    commands.spawn((Camera2d, MainCamera));
    commands.spawn((
        Sprite::from_image(asset_server.load("test7.png")),
        Transform::from_xyz(0., 0., 5.).with_scale(Vec3::new(7., 7., 0.)),
    ));
    commands.spawn((
        Transform::from_xyz(0., 0., 0.),
        Resistor,
        BeginResistor(0),
        EndResistor(0),
    ));
    commands.spawn((
        Transform::from_xyz(0., 10., 0.),
        Resistor,
        EndResistor(0),
        BeginResistor(0),
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
        Transform::from_xyz(-140., 126., -1.),
        Collider::rectangle(1., 1.),
        Sensor,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(170., 40.))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
        Transform::from_xyz(140., 126., -1.),
        Collider::rectangle(1., 1.),
        Sensor,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(345., 40.))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
        Transform::from_xyz(-140., -126., -1.),
        Collider::rectangle(1., 1.),
        Sensor,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(40., 245.))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
        Transform::from_xyz(140., -126., -1.),
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
    if num_of_electrons.num < 27 {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(7.))),
            Collider::circle(1.),
            MeshMaterial2d(materials.add(Color::srgb(0., 1., 0.))),
            Transform::from_xyz(0., 126., 1.),
            Direction(1., 0., 0., 0., 1.),
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
        transform.translation.x += -30.0 * time.delta_secs() * direction.0 * direction.4;
        transform.translation.y += -30.0 * time.delta_secs() * direction.1 * direction.4;
        transform.translation.x += 30.0 * time.delta_secs() * direction.2 * direction.4;
        transform.translation.y += 30.0 * time.delta_secs() * direction.3 * direction.4;
    }
}

fn change_direction_or_speed(
    mut electron_query: Query<
        (Entity, &mut NumCollisions, &mut Direction),
        (With<Electron>, Without<Resistor>),
    >,
    mut collision_event_reader: EventReader<CollisionStarted>,
    resistor_query: Query<
        (Entity, &BeginResistor, &EndResistor),
        (With<Resistor>, Without<Electron>),
    >,
) {
    let mut collided_electrons = HashSet::new();
    for CollisionStarted(e1, e2) in collision_event_reader.read() {
        collided_electrons.insert(*e1);
        collided_electrons.insert(*e2);
    }
    for (resistor, begin, end) in &resistor_query {
        for (electron, mut numcollisions, mut direction) in &mut electron_query {
            if collided_electrons.contains(&electron) {
                println!("trigger regular");
                if collided_electrons.contains(&resistor) {
                    println!("trigger 1");
                    numcollisions.0 -= 1;
                    if begin.0 == 1 {
                        println!("trigger 1");
                        direction.4 = 0.6;
                    } else if end.0 == 1 {
                        direction.4 = 1.0;
                    }
                }
                println!("trigger regular");
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
}

fn spawn_resistors(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    q_camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = *q_camera;
    let window = window;
    if let Some(position) = window.cursor_position() {
        let world_position = camera
            .viewport_to_world_2d(camera_transform, position)
            .unwrap();

        if buttons.just_pressed(MouseButton::Left)
            && (world_position.x > -145. && world_position.x < -135.)
        {
            println!("{}", world_position.x);
            commands.spawn((
                Sprite::from_image(asset_server.load("resistor1.png")),
                Transform::from_xyz(-140., world_position.y, 0.).with_scale(Vec3::new(7., 7., 0.)),
            ));
            commands.spawn((
                Transform::from_xyz(-140., world_position.y + 20., 0.),
                Collider::rectangle(1., 1.),
                Sensor,
                Resistor,
                BeginResistor(1),
            ));
            commands.spawn((
                Transform::from_xyz(-140., world_position.y - 20., 0.),
                Collider::rectangle(1., 1.),
                Sensor,
                Resistor,
                EndResistor(1),
            ));
        }
    }
}
