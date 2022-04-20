# bevy-runner - creating a 3D game with rust and bevy

My second 3D game with rust(https://www.rust-lang.org) and the bevy framework(https://bevyengine.org).

I am inspired by the classic endless runners like #TempleRun or #SonicDash but with cars.

Thanks to Kenny https://www.kenney.nl for the assets.

## 1. Step _ load street and car

<img src="img/step1.png" width="320" align="left"><br><br><br><br><br><br><br><br><br><br>


```Rust
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
```

## 2. Step _ move car

<img src="img/step2.gif" width="256" align="left"><br><br><br><br><br><br><br><br>


```Rust
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
```

## 3. Step _ move street

<img src="img/step3.gif" width="256" align="left"><br><br><br><br><br><br><br><br>


```Rust
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
```
