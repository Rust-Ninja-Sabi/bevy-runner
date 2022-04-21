use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2,PI};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Street;

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
    time:Res<Time>,
    mut position: Query<&mut Transform,With<Street>>
){
    for mut transform in position.iter_mut() {
        transform.translation = transform.translation + Vec3::new(0.0,0.0,1.0) * STREET_SPEED * time.delta_seconds();
        if transform.translation.z > 2.0 {
            transform.translation.z -= 11.0;
        }
    }
}