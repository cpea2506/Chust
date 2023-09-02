use bevy::prelude::*;

use super::assets::PieceAssets;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Resource, Default)]
pub struct SelectedPiece {
    pub entity: Option<Entity>,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
    pub position: Position,
}

/// Returns `None` if square is empty, returns a `Some` with the color if not
fn color_of_square(position: Position, pieces: &Vec<Piece>) -> Option<PieceColor> {
    for piece in pieces {
        if piece.position.x == position.x && piece.position.y == position.y {
            return Some(piece.color);
        }
    }
    None
}

fn is_path_empty(begin: Position, end: Position, pieces: &Vec<Piece>) -> bool {
    // Same column
    if begin.x == end.x {
        for piece in pieces {
            if piece.position.x == begin.x
                && ((piece.position.y > begin.y && piece.position.y < end.y)
                    || (piece.position.y > end.y && piece.position.y < begin.y))
            {
                return false;
            }
        }
    }

    // Same row
    if begin.y == end.y {
        for piece in pieces {
            if piece.position.y == begin.y
                && ((piece.position.x > begin.x && piece.position.x < end.x)
                    || (piece.position.x > end.x && piece.position.x < begin.x))
            {
                return false;
            }
        }
    }

    // Diagonals
    let x_diff = (begin.x as i8 - end.x as i8).abs();
    let y_diff = (begin.y as i8 - end.y as i8).abs();

    if x_diff == y_diff {
        for i in 1..x_diff {
            let pos = if begin.x < end.x && begin.y < end.y {
                // left bottom - right top
                Position::new(begin.x + i as u8, begin.y + i as u8)
            } else if begin.x < end.x && begin.y > end.y {
                // left top - right bottom
                Position::new(begin.x + i as u8, begin.y - i as u8)
            } else if begin.x > end.x && begin.y < end.y {
                // right bottom - left top
                Position::new(begin.x - i as u8, begin.y + i as u8)
            } else {
                // right top - left bottom
                Position::new(begin.x - i as u8, begin.y - i as u8)
            };

            if color_of_square(pos, pieces).is_some() {
                return false;
            }
        }
    }

    true
}

impl Piece {
    pub fn new(piece_type: PieceType, color: PieceColor, position: Position) -> Self {
        Self {
            piece_type,
            color,
            position,
        }
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        mesh: Handle<Mesh>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        piece_assets: &Res<PieceAssets>,
    ) {
        let texture = piece_assets.texture_from(self.piece_type, self.color);

        commands
            .spawn(PbrBundle {
                mesh,
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(texture),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                }),
                transform: {
                    let transform =
                        Transform::from_xyz(self.position.x as f32, 0., self.position.y as f32);

                    transform.with_rotation(Quat::from_rotation_y(match self.color {
                        PieceColor::Black => 90f32.to_radians(),
                        PieceColor::White => -90f32.to_radians(),
                    }))
                },
                ..default()
            })
            .insert(self);
    }

    /// Returns the possible_positions that are available
    pub fn is_move_valid(&self, new_position: Position, pieces: Vec<Piece>) -> bool {
        // If there's a piece of the same color in the same square, it can't move
        if color_of_square(new_position, &pieces) == Some(self.color) {
            return false;
        }

        match self.piece_type {
            PieceType::King => {
                // Horizontal
                ((self.position.x as i8 - new_position.x as i8).abs() == 1
                    && (self.position.y == new_position.y))
                // Vertical
                || ((self.position.y as i8 - new_position.y as i8).abs() == 1
                    && (self.position.x == new_position.x))
                // Diagonal
                || ((self.position.x as i8 - new_position.x as i8).abs() == 1
                    && (self.position.y as i8 - new_position.y as i8).abs() == 1)
            }
            PieceType::Queen => {
                is_path_empty(
                    Position::new(self.position.x, self.position.y),
                    new_position,
                    &pieces,
                ) && ((self.position.x as i8 - new_position.x as i8).abs()
                    == (self.position.y as i8 - new_position.y as i8).abs()
                    || ((self.position.x == new_position.x && self.position.y != new_position.y)
                        || (self.position.y == new_position.y
                            && self.position.x != new_position.x)))
            }
            PieceType::Bishop => {
                is_path_empty(
                    Position::new(self.position.x, self.position.y),
                    new_position,
                    &pieces,
                ) && (self.position.x as i8 - new_position.x as i8).abs()
                    == (self.position.y as i8 - new_position.y as i8).abs()
            }
            PieceType::Knight => {
                ((self.position.x as i8 - new_position.x as i8).abs() == 2
                    && (self.position.y as i8 - new_position.y as i8).abs() == 1)
                    || ((self.position.x as i8 - new_position.x as i8).abs() == 1
                        && (self.position.y as i8 - new_position.y as i8).abs() == 2)
            }
            PieceType::Rook => {
                is_path_empty(
                    Position::new(self.position.x, self.position.y),
                    new_position,
                    &pieces,
                ) && ((self.position.x == new_position.x && self.position.y != new_position.y)
                    || (self.position.y == new_position.y && self.position.x != new_position.x))
            }
            PieceType::Pawn => {
                if self.color == PieceColor::White {
                    // Normal move
                    if new_position.x as i8 - self.position.x as i8 == 1
                        && (self.position.y == new_position.y)
                        && color_of_square(new_position, &pieces).is_none()
                    {
                        return true;
                    }

                    // Move 2 squares
                    if self.position.x == 1
                        && new_position.x as i8 - self.position.x as i8 == 2
                        && (self.position.y == new_position.y)
                        && is_path_empty(
                            Position::new(self.position.x, self.position.y),
                            new_position,
                            &pieces,
                        )
                        && color_of_square(new_position, &pieces).is_none()
                    {
                        return true;
                    }

                    // Take piece
                    if new_position.x as i8 - self.position.x as i8 == 1
                        && (self.position.y as i8 - new_position.y as i8).abs() == 1
                        && color_of_square(new_position, &pieces) == Some(PieceColor::Black)
                    {
                        return true;
                    }
                } else {
                    // Normal move
                    if new_position.x as i8 - self.position.x as i8 == -1
                        && (self.position.y == new_position.y)
                        && color_of_square(new_position, &pieces).is_none()
                    {
                        return true;
                    }

                    // Move 2 squares
                    if self.position.x == 6
                        && new_position.x as i8 - self.position.x as i8 == -2
                        && (self.position.y == new_position.y)
                        && is_path_empty(
                            Position::new(self.position.x, self.position.y),
                            new_position,
                            &pieces,
                        )
                        && color_of_square(new_position, &pieces).is_none()
                    {
                        return true;
                    }

                    // Take piece
                    if new_position.x as i8 - self.position.x as i8 == -1
                        && (self.position.y as i8 - new_position.y as i8).abs() == 1
                        && color_of_square(new_position, &pieces) == Some(PieceColor::White)
                    {
                        return true;
                    }
                }

                false
            }
        }
    }
}
