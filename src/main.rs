use bevy::prelude::*;
use bevy::ui::AlwaysUpdate;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update)
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

fn update(
    mut commands: Commands, 
    input: Res<Input<KeyCode>>,
    always_update: Option<Res<AlwaysUpdate>>,
) {
    if input.just_pressed(KeyCode::Space) {
        if always_update.is_some() {
            commands.remove_resource::<AlwaysUpdate>();
        } else {
            commands.insert_resource(AlwaysUpdate);
        }
    }
}