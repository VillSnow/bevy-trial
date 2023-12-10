use bevy::prelude::*;
use ferris::FerrisDamageEvent;

mod bullet;
mod ferris;
mod logo;

use bullet::SpawnBulletEvent;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (1000.0, 1000.0).into(),
            ..default()
        }),
        ..default()
    }));

    app.add_event::<FerrisDamageEvent>();
    app.add_event::<SpawnBulletEvent>();
    app.add_systems(Startup, setup);
    app.add_systems(Startup, ferris::spawn_ferris);
    app.add_systems(Update, ferris::update_ferris);
    app.add_systems(Update, ferris::hit_test);
    app.add_systems(Update, ferris::drop_out_system);
    app.add_systems(Startup, logo::spawn_logo);
    app.add_systems(Update, logo::update_logo);
    app.add_systems(Update, bullet::spawn_bullet);
    app.add_systems(Update, bullet::update_bullet);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
