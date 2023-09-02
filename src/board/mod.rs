pub mod component;
mod constants;
mod event;

use crate::{pieces::component::*, taken::Taken, turn::PlayerTurn};

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use component::*;
use constants::*;

use event::ResetSelectedEvent;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_event::<ResetSelectedEvent>()
            .add_systems(Startup, create_board)
            .add_systems(Update, (move_pieces, select_piece, reset_selected).chain());
    }
}

pub fn reset_selected(
    mut event_reader: EventReader<ResetSelectedEvent>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
) {
    for _ in event_reader.iter() {
        selected_square.entity = None;
        selected_piece.entity = None;
    }
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane::default()));

    for i in 0..8 {
        for j in 0..8 {
            commands
                .spawn((
                    PbrBundle {
                        mesh: mesh.clone(),
                        material: if (i + j + 1) % 2 == 0 {
                            materials.add(LIGHT_YELLOW.into())
                        } else {
                            materials.add(YELLOW.into())
                        },
                        transform: Transform::from_xyz(i as f32, 0., j as f32),
                        ..default()
                    },
                    PickableBundle::default(),
                    RaycastPickTarget::default(),
                    On::<Pointer<Click>>::run(select_square),
                    HIGHLIGHT_TINT,
                ))
                .insert(Square { x: i, y: j });
        }
    }
}

fn select_square(
    event: Listener<Pointer<Click>>,
    squares_query: Query<&Square>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
) {
    if squares_query.get(event.target).is_ok() {
        selected_square.entity = Some(event.target);
    } else {
        selected_square.entity = None;
        selected_piece.entity = None;
    }
}

fn move_pieces(
    mut commands: Commands,
    selected_square: Res<SelectedSquare>,
    selected_piece: Res<SelectedPiece>,
    mut turn: ResMut<PlayerTurn>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
    mut reset_selected_event: EventWriter<ResetSelectedEvent>,
) {
    if !selected_square.is_changed() {
        return;
    }

    let square_entity = if let Some(entity) = selected_square.entity {
        entity
    } else {
        return;
    };

    let square = if let Ok(square) = squares_query.get(square_entity) {
        square
    } else {
        return;
    };

    if let Some(selected_piece_entity) = selected_piece.entity {
        let pieces_vec: Vec<Piece> = pieces_query.iter_mut().map(|(_, piece)| *piece).collect();
        let pieces_entity_vec: Vec<(Entity, Piece)> = pieces_query
            .iter_mut()
            .map(|(entity, piece)| (entity, *piece))
            .collect();

        if let Ok((_, mut piece)) = pieces_query.get_mut(selected_piece_entity) {
            if piece.is_move_valid(Position::new(square.x, square.y), pieces_vec) {
                // Check if a piece of the opposite color exists in this square and despawn it
                for &(other_entity, other_piece) in &pieces_entity_vec {
                    if other_piece.position.x == square.x
                        && other_piece.position.y == square.y
                        && other_piece.color != piece.color
                    {
                        commands.entity(other_entity).insert(Taken);
                    }
                }
                piece.position.x = square.x;
                piece.position.y = square.y;

                turn.change();
            }

            reset_selected_event.send(ResetSelectedEvent);
        }
    }
}

fn select_piece(
    selected_square: Res<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    turn: Res<PlayerTurn>,
    squares_query: Query<&Square>,
    pieces_query: Query<(Entity, &Piece)>,
) {
    if !selected_square.is_changed() {
        return;
    }

    let square_entity = if let Some(entity) = selected_square.entity {
        entity
    } else {
        return;
    };

    let square = if let Ok(square) = squares_query.get(square_entity) {
        square
    } else {
        return;
    };

    if selected_piece.entity.is_none() {
        // Select the piece in the currently selected square
        for (piece_entity, piece) in pieces_query.iter() {
            if piece.position.x == square.x && piece.position.y == square.y && piece.color == turn.0
            {
                // piece_entity is now the entity in the same square
                selected_piece.entity = Some(piece_entity);
                break;
            }
        }
    }
}
