use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const ENEMY_NUMBER: i32 = 4;
pub const ENEMY_SPEED: f32 = 250.0;
pub const ENEMY_SIZE: f32 = 64.0;
fn main() {
     App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(yep_camera)
        .add_startup_system(yep_player)
        .add_startup_system(yep_enemy)
        .add_system(yep_move)
        .add_system(stay_on_screen_challenge)
        .add_system(yep_move_enemy)
        .add_system(enemy_stay_on_screen_challenge)
        .add_system(confine_enemy)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub enemy_direction: Vec2,
}


pub fn yep_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::new(0.25,0.25,1.0)),
             
            texture: asset_server.load("1image.png"),
            ..default()
        },
        Player {},
    ));
}

/* pub fn enemy_size(
    mut query: Query<&mut Sprite>,
    mut transform_query: Query<&mut Transform>,
){ 

} */
pub fn yep_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
     
    
    for _ in 0..ENEMY_NUMBER {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle{
                    transform: Transform::from_xyz(random_x, random_y, 0.0)
                        .with_scale(Vec3::new(0.15,0.15,1.0)),
                    texture: asset_server.load("2image.png"),
                    ..default()
                }, 
                Enemy {
                    enemy_direction: Vec2::new(random::<f32>(), random::<f32>()). normalize(),
                }, 
        ));
    }}


pub fn yep_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
     commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}


fn yep_move(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard.pressed(KeyCode::W) {
            direction += Vec3::new(0.0,1.0 ,0.0 ) * PLAYER_SPEED * time.delta_seconds()
        }
        if keyboard.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0,0.0 ,0.0 ) * PLAYER_SPEED * time.delta_seconds()
        }
        if keyboard.pressed(KeyCode::S) {
            direction += Vec3::new(0.0,-1.0 ,0.0 ) * PLAYER_SPEED * time.delta_seconds()
        }
        if keyboard.pressed(KeyCode::D) {
            direction += Vec3::new(1.0,0.0 ,0.0 ) * PLAYER_SPEED * time.delta_seconds()
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        } 
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn stay_on_screen_challenge(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
){
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let player_size_by2 = PLAYER_SIZE / 2.0; //32.0
        let x_min = 0.0 + player_size_by2;
        let x_max = window.width() - player_size_by2;
        let y_min = 0.0 + player_size_by2;
        let y_max = window.height() - player_size_by2;

        let mut translation = player_transform.translation;

        if translation.x < x_min{
            translation.x = x_min;
        } else if translation.x > x_max{
            translation.x = x_max;
        }

        if translation.y < y_min{
            translation.y = y_min;
        } else if translation.y > y_max{
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn yep_move_enemy(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let enemy_direction = Vec3::new(enemy.enemy_direction.x, enemy.enemy_direction.y, 0.0);
        transform.translation += enemy_direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn enemy_stay_on_screen_challenge(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let enemy_size_by2 = ENEMY_SIZE / 2.0; //32.0
    let x_min = 0.0 + enemy_size_by2;
    let x_max = window.width() - enemy_size_by2;
    let y_min = 0.0 + enemy_size_by2;
    let y_max = window.height() - enemy_size_by2;

        for (transform, mut enemy) in enemy_query.iter_mut() {
            let translation = transform.translation;
            if translation.x < x_min || translation.x > x_max {
                enemy.enemy_direction.x *= -1.0;
            }
            if translation.y < y_min || translation.y > y_max {
                enemy.enemy_direction.y *= -1.0;
            }
        }
}

pub fn confine_enemy(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let enemy_size_by2 = ENEMY_SIZE / 2.0; //32.0
        let x_min = 0.0 + enemy_size_by2;
        let x_max = window.width() - enemy_size_by2;
        let y_min = 0.0 + enemy_size_by2;
        let y_max = window.height() - enemy_size_by2;


    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        //bound enemy x 
        if translation.x < x_min{
            translation.x = x_min;
        } else if translation.x > x_max{
            translation.x = x_max;
        }
        
        //bound enemy y
        if translation.y < y_min{
            translation.y = y_min;
        } else if translation.y > y_max{
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}