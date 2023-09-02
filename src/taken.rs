use bevy::{app::AppExit, prelude::*};

use crate::pieces::component::{Piece, PieceColor, PieceType};

#[derive(Component)]
pub struct Taken;

pub struct TakenPlugin;

impl Plugin for TakenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_taken_piece);
    }
}

fn despawn_taken_piece(
    mut commands: Commands,
    mut app_exit_events: EventWriter<AppExit>,
    query: Query<(Entity, &Piece), With<Taken>>,
) {
    for (entity, piece) in query.iter() {
        if piece.piece_type == PieceType::King {
            println!(
                "{} won! Thanks for playing!",
                match piece.color {
                    PieceColor::White => "White",
                    PieceColor::Black => "Black",
                }
            );

            app_exit_events.send(AppExit);
        }

        commands.entity(entity).despawn();
    }
}
