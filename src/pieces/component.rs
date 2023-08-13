use bevy::prelude::*;

use super::assets::PieceAssets;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Component)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
    pub position: Vec2,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: PieceColor, position: Vec2) -> Self {
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
                    let transform = Transform::from_xyz(self.position.x, 0., self.position.y);

                    transform.with_rotation(Quat::from_rotation_y(match self.color {
                        PieceColor::Black => 90f32.to_radians(),
                        PieceColor::White => -90f32.to_radians(),
                    }))
                },
                ..default()
            })
            .insert(self);
    }
}
