use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((NodeBundle {
            background_color: Color::RED.into(),
            ..Default::default()
        },
        CalculatedSize::default()
    ));
}
