mod assets;
pub mod component;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use assets::PieceAssets;
use component::*;

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<PieceAssets>()
            .add_systems(Startup, (create_white_pieces, create_black_pieces))
            .add_systems(Update, move_piece);
    }
}

fn move_piece(mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        transform.translation = Vec3::new(piece.position.x as f32, 0., piece.position.y as f32);
    }
}

fn create_black_pieces(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    piece_assets: Res<PieceAssets>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane::default()));

    Piece::new(PieceType::Rook, PieceColor::Black, Position::new(7, 0)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Knight, PieceColor::Black, Position::new(7, 1)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Bishop, PieceColor::Black, Position::new(7, 2)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Queen, PieceColor::Black, Position::new(7, 3)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::King, PieceColor::Black, Position::new(7, 4)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Bishop, PieceColor::Black, Position::new(7, 5)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Knight, PieceColor::Black, Position::new(7, 6)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Rook, PieceColor::Black, Position::new(7, 7)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    // pawn
    for i in 0..8 {
        Piece::new(PieceType::Pawn, PieceColor::Black, Position::new(6, i)).spawn(
            &mut commands,
            mesh.clone(),
            &mut materials,
            &piece_assets,
        );
    }
}

fn create_white_pieces(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    piece_assets: Res<PieceAssets>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane::default()));

    Piece::new(PieceType::Rook, PieceColor::White, Position::new(0, 0)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Knight, PieceColor::White, Position::new(0, 1)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Bishop, PieceColor::White, Position::new(0, 2)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Queen, PieceColor::White, Position::new(0, 3)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::King, PieceColor::White, Position::new(0, 4)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Bishop, PieceColor::White, Position::new(0, 5)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Knight, PieceColor::White, Position::new(0, 6)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Rook, PieceColor::White, Position::new(0, 7)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    for i in 0..8 {
        Piece::new(PieceType::Pawn, PieceColor::White, Position::new(1, i)).spawn(
            &mut commands,
            mesh.clone(),
            &mut materials,
            &piece_assets,
        );
    }
}
