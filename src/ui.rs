use crate::{pieces::component::PieceColor, turn::PlayerTurn};
use bevy::prelude::*;

// Component to mark the Text entity
#[derive(Component)]
struct NextMoveText;

/// Initialize UiCamera and text
fn init_next_move_text(mut commands: Commands) {
    commands.spawn(UiCameraConfig::default());

    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                top: Val::Px(10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text::from_section(
                        "Next move: White".to_string(),
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(NextMoveText);
        });
}

/// Update text with the correct turn
fn next_move_text_update(turn: Res<PlayerTurn>, mut query: Query<(&mut Text, With<NextMoveText>)>) {
    if turn.is_changed() {
        for (mut text, _) in query.iter_mut() {
            if let Some(section) = text.sections.first_mut() {
                section.value = format!(
                    "Next move: {}",
                    match turn.0 {
                        PieceColor::White => "White",
                        PieceColor::Black => "Black",
                    }
                );
            }
        }
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_next_move_text)
            .add_systems(Update, next_move_text_update);
    }
}
