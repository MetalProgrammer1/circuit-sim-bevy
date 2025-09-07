use avian2d::prelude::*;
use bevy::prelude::Hsla;
use bevy::text::FontSmoothing;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use rand::Rng;
use std::time::Duration;
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
        .add_systems(FixedUpdate, change_direction_or_speed)
        .add_systems(
            Update,
            spawn_electrons.run_if(on_timer(Duration::from_secs(1))),
        )
        .add_systems(Update, (move_electrons, spawn_resistors, switch).chain())
        .add_systems(Update, change_led.run_if(on_timer(Duration::from_secs(1))))
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
struct Wall;

#[derive(Component)]
struct ResistorId(i32);

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

#[derive(Resource)]
struct PowerOfLed(f32);

#[derive(Component)]
struct Led;

#[derive(Component)]
struct Perm;

#[derive(Resource)]
struct NumResistors {
    num: i32,
}

#[derive(Resource)]
struct OnOrOff(i32);

#[derive(Component)]
struct ResistorsHit {
    id_store: Vec<i32>,
}
#[derive(Component)]
struct DelText;
#[derive(Component)]
struct MainCamera;

fn set_up(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PowerOfLed(0.0));
    commands.insert_resource(OnOrOff(1));
    commands.insert_resource(NumOfElectrons { num: 0 });
    commands.insert_resource(NumResistors { num: 0 });
    commands.spawn((Camera2d, MainCamera));
    commands.spawn((
        Sprite::from_image(asset_server.load("circuit.png")),
        Transform::from_xyz(0., 0., 5.).with_scale(Vec3::new(7., 7., 0.)),
    ));
    commands.spawn((Transform::from_xyz(0., 0., 0.), Resistor, ResistorId(0)));
    commands.spawn((
        Transform::from_xyz(140., 40., 10.),
        Collider::rectangle(1., 1.),
        Led,
        Perm,
    ));
}

fn set_up_sensors(
    mut commands: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        //Mesh2d(meshes.add(Rectangle::new(40., 245.))),
        //MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
        Transform::from_xyz(-140., 126., -1.),
        Collider::rectangle(1., 1.),
        Sensor,
        Wall,
    ));
    commands.spawn((
        //Mesh2d(meshes.add(Rectangle::new(170., 40.))),
        //MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
        Transform::from_xyz(140., 126., -1.),
        Collider::rectangle(1., 1.),
        Sensor,
        Wall,
    ));
    commands.spawn((
        //Mesh2d(meshes.add(Rectangle::new(345., 40.))),
        //MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
        Transform::from_xyz(-140., -126., -1.),
        Collider::rectangle(1., 1.),
        Sensor,
        Wall,
    ));
    commands.spawn((
        //Mesh2d(meshes.add(Rectangle::new(40., 245.))),
        //MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
        Transform::from_xyz(140., -126., -1.),
        Collider::rectangle(1., 1.),
        Sensor,
        Wall,
    ));
}

fn spawn_electrons(
    mut num_of_electrons: ResMut<NumOfElectrons>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    onoroff: ResMut<OnOrOff>,
) {
    if num_of_electrons.num < 35 && onoroff.0 == 1 {
        let num = rand::thread_rng().gen_range(-10..10);
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(7.))),
            Collider::circle(1.),
            MeshMaterial2d(materials.add(Color::srgb(0., 1., 0.))),
            Transform::from_xyz(num as f32 * 0.01 * num_of_electrons.num as f32, 126., 1.),
            Direction(1., 0., 0., 0., 1.),
            NumCollisions(0),
            Electron,
            Sensor,
            CollisionEventsEnabled,
            ResistorsHit {
                id_store: Vec::new(),
            },
        ));
    }
    num_of_electrons.num += 1;
}

fn move_electrons(
    mut query: Query<(&mut Transform, &Direction), With<Electron>>,
    time: Res<Time>,
    onoroff: ResMut<OnOrOff>,
) {
    for (mut transform, direction) in &mut query {
        transform.translation.x +=
            -30.0 * time.delta_secs() * direction.0 * direction.4 * onoroff.0 as f32;
        transform.translation.y +=
            -30.0 * time.delta_secs() * direction.1 * direction.4 * onoroff.0 as f32;
        transform.translation.x +=
            30.0 * time.delta_secs() * direction.2 * direction.4 * onoroff.0 as f32;
        transform.translation.y +=
            30.0 * time.delta_secs() * direction.3 * direction.4 * onoroff.0 as f32;
    }
}

fn change_direction_or_speed(
    mut electron_query: Query<
        (
            Entity,
            &mut NumCollisions,
            &mut Direction,
            &mut ResistorsHit,
        ),
        (With<Electron>, Without<Resistor>),
    >,
    mut pow: ResMut<PowerOfLed>,
    mut collision_event_reader: EventReader<CollisionStarted>,
    resistor_query: Query<(Entity, &ResistorId), (With<Resistor>, Without<Electron>)>,
    led_query: Query<Entity, (With<Led>, Without<Resistor>, Without<Electron>)>,
    mut wall_query: Query<
        Entity,
        (
            With<Wall>,
            Without<Led>,
            Without<Resistor>,
            Without<Electron>,
        ),
    >,
) {
    for CollisionStarted(e1, e2) in collision_event_reader.read() {
        let (electron_entity, other_entity) = if let Ok((e, _, _, _)) = electron_query.get(*e1) {
            (e, *e2)
        } else if let Ok((e, _, _, _)) = electron_query.get(*e2) {
            (e, *e1)
        } else {
            continue;
        };

        if let Ok((_, mut numcollisions, mut direction, mut store)) =
            electron_query.get_mut(electron_entity)
        {
            if let Ok((_, id_num)) = resistor_query.get(other_entity) {
                if !store.id_store.contains(&id_num.0) {
                    direction.4 *= 0.8;
                    store.id_store.push(id_num.0);
                }
            }

            if !resistor_query.get(other_entity).is_ok()
                && !led_query.get(other_entity).is_ok()
                && wall_query.get_mut(other_entity).is_ok()
            {
                numcollisions.0 += 1;
                println!("trigger regular");
            }
            if led_query.get(other_entity).is_ok() {
                if direction.4 < pow.0 || direction.4 > pow.0 {
                    pow.0 = direction.4;
                }
            }
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

fn spawn_resistors(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    q_camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut num: ResMut<NumResistors>,
) {
    let (camera, camera_transform) = *q_camera;
    let window = window;
    if let Some(position) = window.cursor_position() {
        let world_position = camera
            .viewport_to_world_2d(camera_transform, position)
            .unwrap();

        if buttons.just_pressed(MouseButton::Left)
            && ((world_position.x > -145. && world_position.x < -135.)
                || (world_position.x < 145. && world_position.x > 135.)
                || (world_position.y > -131. && world_position.y < -121.))
        {
            num.num += 1;
            if world_position.x > -145. && world_position.x < -135. {
                commands.spawn((
                    Sprite::from_image(asset_server.load("resistor.png")),
                    Transform::from_xyz(-140., world_position.y, 0.)
                        .with_scale(Vec3::new(7., 7., 0.)),
                ));
                commands.spawn((
                    Transform::from_xyz(-140., world_position.y, 0.),
                    Collider::rectangle(1., 1.),
                    Sensor,
                    Resistor,
                    ResistorId(num.num),
                    CollisionEventsEnabled,
                ));
            } else if world_position.x < 145. && world_position.x > 135. {
                commands.spawn((
                    Sprite::from_image(asset_server.load("resistor.png")),
                    Transform::from_xyz(140., world_position.y, 0.)
                        .with_scale(Vec3::new(7., 7., 0.)),
                ));
                commands.spawn((
                    Transform::from_xyz(140., world_position.y, 0.),
                    Collider::rectangle(1., 1.),
                    Sensor,
                    Resistor,
                    ResistorId(num.num),
                    CollisionEventsEnabled,
                ));
            } else if world_position.y > -131. && world_position.y < -121. {
                println!("resistor should go here :(");
                commands.spawn((
                    Sprite::from_image(asset_server.load("resistor.png")),
                    Transform::from_xyz(world_position.x, -126., 0.)
                        .with_scale(Vec3::new(7., 7., 0.)),
                ));
                commands.spawn((
                    Transform::from_xyz(world_position.x, -126., 0.),
                    Collider::rectangle(1., 1.),
                    Sensor,
                    Resistor,
                    ResistorId(num.num),
                    CollisionEventsEnabled,
                ));
            };
        }
    }
}

fn change_led(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    pow: ResMut<PowerOfLed>,
    q1: Query<Entity, (With<Led>, Without<Perm>)>,
    onoroff: ResMut<OnOrOff>,
) {
    for entity in q1.iter() {
        commands.entity(entity).despawn();
    }
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(20.))),
        MeshMaterial2d(materials.add(Color::Hsla(Hsla::hsl(
            0.0,
            0.9,
            0.9 * pow.0 * onoroff.0 as f32,
        )))),
        Transform::from_xyz(140., 40., 10.),
        Led,
    ));
}

fn switch(
    mut commands: Commands,
    assets: Res<AssetServer>,
    buttons: Res<ButtonInput<KeyCode>>,
    mut check: ResMut<OnOrOff>,
    q1: Query<Entity, With<DelText>>,
) {
    let font = assets.load("fonts/FiraSans.ttf");
    for entity in q1.iter() {
        commands.entity(entity).despawn();
    }
    if check.0 == 1 {
        commands.spawn((
            Text2d::new("On"),
            TextColor(Color::WHITE),
            TextFont::from_font(font.clone()).with_font_size(60.),
            Transform::from_xyz(0., 0., 0.),
            TextLayout::new_with_justify(JustifyText::Center),
            DelText,
        ));
    } else if check.0 == 0 {
        commands.spawn((
            Text2d::new("Off"),
            TextColor(Color::WHITE),
            TextFont::from_font(font.clone()).with_font_size(60.),
            Transform::from_xyz(0., 0., 0.),
            TextLayout::new_with_justify(JustifyText::Center),
            DelText,
        ));
    };
    if check.0 == 1 && buttons.just_pressed(KeyCode::Space) {
        check.0 = 0;
    } else if check.0 == 0 && buttons.just_pressed(KeyCode::Space) {
        check.0 = 1;
    };
}
