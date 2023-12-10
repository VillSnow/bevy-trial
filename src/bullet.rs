use bevy::prelude::*;

#[derive(Event)]
pub struct SpawnBulletEvent {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub speed: f32,
    pub margin: f32,
}

#[derive(Component, Default)]
pub struct Bullet {
    pub u: f32,
    pub v: f32,
    pub radius: f32,
}

pub fn spawn_bullet(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<SpawnBulletEvent>,
) {
    for event in events.iter() {
        let x = event.x + event.angle.cos() * event.margin;
        let y = event.y + event.angle.sin() * event.margin;

        let u = event.angle.cos() * event.speed;
        let v = event.angle.sin() * event.speed;

        command.spawn((
            SpriteBundle {
                texture: asset_server.load("bullet.png"),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0))
                    .with_scale(Vec3::splat(5.0 / 330.0)),
                ..default()
            },
            Bullet { u, v, radius: 5.0 },
        ));
    }
}

pub fn update_bullet(time: Res<Time>, mut bullets: Query<(&mut Transform, &Bullet)>) {
    for bullet in bullets.iter_mut() {
        let (mut bullet_transform, bullet_bullet) = bullet;
        bullet_transform.translation.x += bullet_bullet.u * time.delta_seconds();
        bullet_transform.translation.y += bullet_bullet.v * time.delta_seconds();
    }
}
