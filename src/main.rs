use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::f32::consts::{FRAC_PI_2,PI};
use bevy::core::FixedTimestep;

const OBSTACLE_MODELS: &'static [&'static str] = &["models/hatchbackSports.glb#Scene0", "models/police.glb#Scene0",
                                                    "models/sedan.glb#Scene0","models/tractor.glb#Scene0"];
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct Street;

#[derive(Component)]
struct Coin;

#[derive(Component)]
struct Cointext;

#[derive(Component)]
struct Besttext;

struct Score {
    value:i32,
    best:i32
}
impl Default for Score{
    fn default() -> Self {
        Self {
            value:0,
            best:0,
        }
    }
}

fn main() {
    App::new()
        //add config resources
        .insert_resource(Msaa {samples: 4})
        .insert_resource(WindowDescriptor{
            title: "bevy-runner".to_string(),
            width: 400.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(Score::default())
        //.insert_resource(Gamegrid::default())
        //bevy itself
        .add_plugins(DefaultPlugins)
        //.add_state(Gamestate::Play)
        //.add_plugin(ConfigCam)
        //resources
        //.insert_resource(Score{value:0})
        // if it implements `Default` or `FromWorld`
        //.init_resource::<MyFancyResource>()
        // events:
        //.add_event::<LevelUpEvent>()
        // system once
        .add_startup_system(setup)
        // system frame
        .add_system(move_car)
        .add_system(move_street)
        .add_system(move_coin)
        .add_system(move_obstacle)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(4.0))
                .with_system(spawn_obstacle)
        )
        .add_system_to_stage(CoreStage::PostUpdate, collision_coin)
        .add_system_to_stage(CoreStage::Last, scoreboard)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){

    //camera
    commands.spawn_bundle(PerspectiveCameraBundle{
        transform: Transform::from_xyz(1.0,6.0,3.0).looking_at(Vec3::new(1.,0.,-2.), Vec3::Y),
        ..Default::default()
    });

    commands.spawn_bundle(UiCameraBundle::default());
    // light
    commands.spawn_bundle(PointLightBundle{
        point_light: PointLight{
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(1.0, 4.0, 0.0),
        ..Default::default()
    });

    // street
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..3);

    for j in -9..2 {
        let mut children_list:Vec<Entity> = Vec::new();
        for i in 0..3 {
            let entity = commands.spawn_bundle((
                Transform {
                    translation: Vec3::new(i as f32, 0.0, 0.0),
                    rotation: Quat::from_rotation_y(FRAC_PI_2),
                    ..Default::default()
                },
                GlobalTransform::identity(),
            ))
                .with_children(|parent| {
                    parent.spawn_scene(asset_server.load("models/road_straight.glb#Scene0"));
                }).id();
            children_list.push(entity);
            if i == 0 {
                let lamp = commands.spawn_bundle((
                    Transform {
                        translation: Vec3::new(i as f32-0.45, 0.0, 0.0),
                        rotation: Quat::from_rotation_y(FRAC_PI_2),
                        ..Default::default()
                    },
                    GlobalTransform::identity(),
                ))
                    .with_children(|parent| {
                        parent.spawn_scene(asset_server.load("models/lamp.glb#Scene0"));
                    }).id();
                children_list.push(lamp);
            }
            if i == 2 {
                let lamp = commands.spawn_bundle((
                    Transform {
                        translation: Vec3::new(i as f32+0.45, 0.0, 0.0),
                        rotation: Quat::from_rotation_y(-FRAC_PI_2),
                        ..Default::default()
                    },
                    GlobalTransform::identity(),
                ))
                    .with_children(|parent| {
                        parent.spawn_scene(asset_server.load("models/lamp.glb#Scene0"));
                    }).id();
                children_list.push(lamp);
            }
            commands.spawn_bundle((
                Transform {
                    translation: Vec3::new(0.0, 0.0, j as f32),
                    ..Default::default()
                },
                GlobalTransform::identity(),
            )).insert(Street)
            .push_children(&children_list);
        }
        // coin
        if j < -1 {
            let ran_street = die.sample(&mut rng);
            commands.spawn_bundle((
                Transform {
                    translation: Vec3::new(ran_street as f32,0.0,j as f32),
                    scale: Vec3::new(0.5,0.5,0.5),
                    ..Default::default()
                },
                GlobalTransform::identity(),
            ))
                .with_children(|parent| {
                    parent.spawn_scene(asset_server.load("models/coin.glb#Scene0"));
                })
                .insert(Coin);
        }
    }

    //player
    commands.spawn_bundle((
        Transform {
            translation: Vec3::new(1.0,0.0,0.0),
            scale: Vec3::new(0.4, 0.4, 0.4),
            ..Default::default()
        },
        GlobalTransform::identity(),
    ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/taxi.glb#Scene0"));
        })
        .insert(Player);

    // scoreboard
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Coin:",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
            },
            Default::default(),
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).
        insert(Cointext);
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Best:",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
            },
            Default::default(),
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                right: Val::Px(25.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Besttext);

}

fn move_car(
    keyboard_input: Res<Input<KeyCode>>,
    mut position: Query<&mut Transform,With<Player>>
){
    for mut transform in position.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Left){
            let mut x = transform.translation.x-1.0;
            if x < 0.0 { x=0.0};
            transform.translation = Vec3::new(x,
                                         transform.translation.y,
                                         transform.translation.z);
        }
        if keyboard_input.just_pressed(KeyCode::Right){
            let mut x = transform.translation.x+1.0;
            if x > 2.0 { x = 2.0};
            transform.translation = Vec3::new(x,
                                         transform.translation.y,
                                         transform.translation.z);
        }
    }
}

const STREET_SPEED:f32 = 1.5;

fn move_street(
    mut commands: Commands,
    time:Res<Time>,
    mut position: Query<&mut Transform,With<Street>>,
    asset_server: Res<AssetServer>
){
    for mut transform in position.iter_mut() {
        transform.translation = transform.translation + Vec3::new(0.0,0.0,1.0) * STREET_SPEED * time.delta_seconds();
        if transform.translation.z > 2.0 {
            transform.translation.z -= 11.0;
            let mut rng = rand::thread_rng();
            let ran_ = rng.gen_range(0..10);
            if ran_ < 2 {
                let die = Uniform::from(0..3);
                let ran_street = die.sample(&mut rng);
                commands.spawn_bundle((
                    Transform {
                        translation: Vec3::new(ran_street as f32, 0.0, transform.translation.z),
                        scale: Vec3::new(0.5, 0.5, 0.5),
                        ..Default::default()
                    },
                    GlobalTransform::identity(),
                ))
                    .with_children(|parent| {
                        parent.spawn_scene(asset_server.load("models/coin.glb#Scene0"));
                    })
                    .insert(Coin);
            }
        }
    }
}

fn move_coin(
    time:Res<Time>,
    mut commands: Commands,
    mut position: Query<(Entity, &mut Transform), With<Coin>>
){
    for (entity, mut transform) in position.iter_mut() {
        transform.translation = transform.translation + Vec3::new(0.0,0.0,1.0) * STREET_SPEED * time.delta_seconds();
        if transform.translation.z >= 1.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn collision_coin(
    mut commands: Commands,
    mut score: ResMut<Score>,
    position: Query<(Entity, &Transform), With<Coin>>,
    player_position: Query<&Transform,With<Player>>
){
    let player_transfrom = player_position.single();
    for (entity, transform) in position.iter() {
        if transform.translation.x == player_transfrom.translation.x {
            if (transform.translation.z - player_transfrom.translation.z).abs() < 0.4 {
                commands.entity(entity).despawn_recursive();
                score.value += 1;
            }
        }
    }
}

fn scoreboard(
    score: Res<Score>,
    mut coin_query: Query<(&mut Text, With<Cointext>, Without<Besttext>)>,
    mut best_query: Query<&mut Text, With<Besttext>>,
) {
    let (mut text,_,_) = coin_query.single_mut();
    text.sections[0].value = format!("Coin: {}", score.value);

    let mut best_text = best_query.single_mut();
    best_text.sections[0].value = format!("Best: {}", score.best);
}

fn spawn_obstacle(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..3);
    let ran_street = die.sample(&mut rng);

    let model = OBSTACLE_MODELS[rng.gen_range(0..OBSTACLE_MODELS.len())];
    commands.spawn_bundle((
        Transform {
            translation: Vec3::new(ran_street as f32,0.0,-10.0),
            scale: Vec3::new(0.4, 0.4, 0.4),
            rotation: Quat::from_rotation_y(PI)
        },
        GlobalTransform::identity(),
    ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load(model));
        })
        .insert(Obstacle);
    //println!("{}", model);
}

const OBSTACLE_SPEED:f32 = 2.0;

fn move_obstacle(
    time:Res<Time>,
    mut commands: Commands,
    mut position: Query<(Entity, &mut Transform), With<Obstacle>>
){
    for (entity, mut transform) in position.iter_mut() {
        transform.translation = transform.translation + Vec3::new(0.0,0.0,1.0) * OBSTACLE_SPEED * time.delta_seconds();
        if transform.translation.z >= 1.0 {
            commands.entity(entity).despawn_recursive();
            //println!("despawn");
        }
    }
}