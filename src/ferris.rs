use std::time::Duration;

use bevy::prelude::*;

use crate::bullet::Bullet;

#[derive(Component, Default)]
pub struct Ferris {
    state: FerrisState,
}

#[derive(Default)]
enum FerrisState {
    #[default]
    Normal,
    Dropout {
        timer: Timer,
    },
}

#[derive(Event, Default)]
pub struct FerrisDamageEvent;

pub fn spawn_ferris(mut command: Commands, asset_server: Res<AssetServer>) {
    command.spawn((
        Ferris::default(),
        SpriteBundle {
            texture: asset_server.load("rustacean-orig-noshadow.png"),
            transform: Transform::from_xyz(0.0, -250.0, 0.0).with_scale(Vec3::splat(0.1)),
            ..default()
        },
    ));
}

pub fn update_ferris(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut ferris: Query<&mut Transform, With<Ferris>>,
) {
    let mut transform = match ferris.get_single_mut() {
        Ok(x) => x,
        _ => return,
    };

    let speed = 100.0;
    let u = match (keys.pressed(KeyCode::Right), keys.pressed(KeyCode::Left)) {
        (true, false) => speed,
        (false, true) => -speed,
        _ => 0.0,
    };
    let v = match (keys.pressed(KeyCode::Up), keys.pressed(KeyCode::Down)) {
        (true, false) => speed,
        (false, true) => -speed,
        _ => 0.0,
    };

    transform.translation.x += u * time.delta_seconds();
    transform.translation.y += v * time.delta_seconds();
}

pub fn hit_test(
    mut ferris: Query<(&Transform, &mut Ferris)>,
    bullets: Query<(&mut Transform, &Bullet), Without<Ferris>>,
) {
    let (ferris_transform, mut ferris_ferris) = match ferris.get_single_mut() {
        Ok(x) => x,
        _ => return,
    };

    for bullet in bullets.iter() {
        let (bullet_transform, bullet_bullet) = bullet;

        let distance = f32::hypot(
            bullet_transform.translation.x - ferris_transform.translation.x,
            bullet_transform.translation.y - ferris_transform.translation.y,
        );
        if matches!(ferris_ferris.state, FerrisState::Normal) && distance <= bullet_bullet.radius {
            ferris_ferris.state = FerrisState::Dropout {
                timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
            };
        }
    }
}

pub fn drop_out_system(
    mut command: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Ferris)>,
) {
    let (entity, mut transform, mut ferris) = match query.get_single_mut() {
        Ok(x) => x,
        _ => return,
    };
    let timer = match &mut ferris.state {
        FerrisState::Dropout { timer } => timer,
        _ => return,
    };

    timer.tick(time.delta());
    transform.rotation = Quat::from_axis_angle(Vec3::Z, timer.elapsed_secs() * 10.0);
    if timer.just_finished() {
        command.entity(entity).despawn()
    }
}
