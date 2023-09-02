mod component;
mod constants;

use crate::pieces::component::*;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use component::*;
use constants::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_systems(Startup, create_board);
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
    mut commands: Commands,
    event: Listener<Pointer<Click>>,
    squares_query: Query<&Square>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
) {
    if let Ok(square) = squares_query.get(event.target) {
        selected_square.entity = Some(event.target);

        if let Some(selected_piece_entity) = selected_piece.entity {
            let pieces_vec: Vec<Piece> = pieces_query.iter_mut().map(|(_, piece)| *piece).collect();
            let pieces_entity_vec: Vec<(Entity, Piece)> = pieces_query
                .iter_mut()
                .map(|(entity, piece)| (entity, *piece))
                .collect();

            // Move the selected piece to the selected square
            if let Ok((_, mut piece)) = pieces_query.get_mut(selected_piece_entity) {
                if piece.is_move_valid(Position::new(square.x, square.y), pieces_vec) {
                    // Check if a piece of the opposite color exists in this square and despawn it
                    for &(other_entity, other_piece) in &pieces_entity_vec {
                        if other_piece.position.x == square.x
                            && other_piece.position.y == square.y
                            && other_piece.color != piece.color
                        {
                            commands.entity(other_entity).despawn();
                        }
                    }
                    piece.position.x = square.x;
                    piece.position.y = square.y;
                }
            }

            selected_square.entity = None;
            selected_piece.entity = None;
        } else {
            // Select the piece in the currently selected square
            for (piece_entity, piece) in pieces_query.iter_mut() {
                if piece.position.x == square.x && piece.position.y == square.y {
                    // piece_entity is now the entity in the same square
                    selected_piece.entity = Some(piece_entity);
                    break;
                }
            }
        }
    } else {
        selected_square.entity = None;
        selected_piece.entity = None;
    }
}
