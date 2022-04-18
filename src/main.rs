use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2,PI};

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        //add config resources
        .insert_resource(Msaa {samples: 4})
        .insert_resource(WindowDescriptor{
            title: "bevy-runner".to_string(),
            width: 1000.0,
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
        //.add_system(move)
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
    for j in -8..2 {
        for i in 0..3 {
            commands.spawn_bundle((
                Transform {
                    translation: Vec3::new(i as f32, 0.0, j as f32),
                    rotation: Quat::from_rotation_y(FRAC_PI_2),
                    ..Default::default()
                },
                GlobalTransform::identity(),
            ))
                .with_children(|parent| {
                    parent.spawn_scene(asset_server.load("models/road_straight.glb#Scene0"));
                });
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