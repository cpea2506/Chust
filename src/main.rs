mod board;
mod pieces;
mod taken;
mod turn;
mod ui;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use board::BoardPlugin;
use pieces::PiecePlugin;
use taken::TakenPlugin;
use turn::TurnPlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_systems(Startup, setup)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1600., 1600.).into(),
                    title: String::from("Chust"),
                    ..default()
                }),
                ..default()
            }),
            DefaultPickingPlugins
                .build()
                .disable::<DebugPickingPlugin>(),
            BoardPlugin,
            PiecePlugin,
            TurnPlugin,
            UIPlugin,
            TakenPlugin,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        },
        RaycastPickCamera::default(),
    ));

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}
