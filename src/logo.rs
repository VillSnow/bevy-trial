use std::time::Duration;

use bevy::prelude::*;

use crate::{bullet::SpawnBulletEvent, ferris::Ferris};

#[derive(Component)]
pub struct Logo {
    fire_book: FireBook,
}

struct FireBook {
    list: Vec<(Option<FireBookItem>, Duration)>,
    index: usize,
    timer: Timer,
}

#[derive(Clone)]
enum FireBookItem {
    Ways { n: i32, speed: f32, spread: f32 },
    FullCircle { n: i32, angle: f32, speed: f32 },
}

impl FireBook {
    fn tick(&mut self, dur: Duration) -> Vec<FireBookItem> {
        self.timer.tick(dur);
        if self.timer.just_finished() {
            self.index = (self.index + 1) % self.list.len();
            self.timer = Timer::new(self.list[self.index].1.clone(), TimerMode::Once);

            if let Some(value) = &self.list[self.index].0 {
                vec![value.clone()]
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }
}

impl Logo {
    fn new() -> Self {
        let mut list = Vec::new();
        for _ in 0..10 {
            list.push((
                Some(FireBookItem::Ways {
                    n: 3,
                    speed: 200.0,
                    spread: (5.0f32).to_radians(),
                }),
                Duration::from_millis(100),
            ))
        }
        for i in 0..10 {
            list.push((
                Some(FireBookItem::FullCircle {
                    n: 20,
                    angle: (i as f32 * 5.0).to_radians(),
                    speed: 50.0,
                }),
                Duration::from_millis(500),
            ))
        }
        for i in 5..10 {
            list.push((
                Some(FireBookItem::Ways {
                    n: i,
                    speed: 100.0,
                    spread: (5.0f32).to_radians(),
                }),
                Duration::from_millis(500),
            ))
        }
        for i in 0..10 {
            list.push((
                Some(FireBookItem::FullCircle {
                    n: 20,
                    angle: (i as f32 * -5.0).to_radians(),
                    speed: 50.0,
                }),
                Duration::from_millis(500),
            ))
        }

        Self {
            fire_book: FireBook {
                list,
                index: 0,
                timer: Timer::new(Duration::ZERO, TimerMode::Once),
            },
        }
    }
}

pub fn spawn_logo(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("rust-logo-512x512.png"),
            transform: Transform::from_xyz(0.0, 250.0, 1.0).with_scale(Vec3::new(0.1, 0.1, 1.)),
            ..default()
        },
        Logo::new(),
    ));
}

pub fn update_logo(
    time: Res<Time>,
    mut logo: Query<(&mut Transform, &mut Logo)>,
    mut ferris: Query<&Transform, (With<Ferris>, Without<Logo>)>,
    mut fire_events: EventWriter<SpawnBulletEvent>,
) {
    let (logo_transform, mut logo_logo) = logo.single_mut();
    let ferris_transform = ferris.get_single_mut().ok();

    let items = logo_logo.fire_book.tick(time.delta());

    if let Some(ferris_transform) = ferris_transform {
        for item in items {
            match item {
                FireBookItem::Ways { n, speed, spread } => {
                    let angle_zero = f32::atan2(
                        ferris_transform.translation.y - logo_transform.translation.y,
                        ferris_transform.translation.x - logo_transform.translation.x,
                    );

                    for i in 0..n {
                        fire_events.send(SpawnBulletEvent {
                            x: logo_transform.translation.x,
                            y: logo_transform.translation.y,
                            angle: angle_zero - spread * (n - 1) as f32 / 2.0 + spread * i as f32,
                            speed,
                            margin: 50.0,
                        })
                    }
                }
                FireBookItem::FullCircle { n, angle, speed } => {
                    for i in 0..n {
                        fire_events.send(SpawnBulletEvent {
                            x: logo_transform.translation.x,
                            y: logo_transform.translation.y,
                            angle: angle + i as f32 / n as f32 * std::f32::consts::TAU,
                            speed,
                            margin: 100.0,
                        })
                    }
                }
            }
        }
    }
}
