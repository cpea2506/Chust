mod assets;
mod component;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use assets::PieceAssets;
use component::*;

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<PieceAssets>()
            .add_systems(Startup, (create_white_pieces, create_black_pieces));
    }
}

fn create_black_pieces(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    piece_assets: Res<PieceAssets>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane::default()));

    Piece::new(PieceType::Rook, PieceColor::Black, Vec2::new(7., 0.)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Knight, PieceColor::Black, Vec2::new(7., 1.)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Bishop, PieceColor::Black, Vec2::new(7., 2.)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Queen, PieceColor::Black, Vec2::new(7., 3.)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::King, PieceColor::Black, Vec2::new(7., 4.)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Bishop, PieceColor::Black, Vec2::new(7., 5.)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Knight, PieceColor::Black, Vec2::new(7., 6.)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Rook, PieceColor::Black, Vec2::new(7., 7.)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    // pawn
    for i in 0..8 {
        Piece::new(PieceType::Pawn, PieceColor::Black, Vec2::new(6., i as f32)).spawn(
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

    Piece::new(PieceType::Rook, PieceColor::White, Vec2::ZERO).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Knight, PieceColor::White, Vec2::new(0., 1.)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Bishop, PieceColor::White, Vec2::new(0., 2.)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Queen, PieceColor::White, Vec2::new(0., 3.)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::King, PieceColor::White, Vec2::new(0., 4.)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Bishop, PieceColor::White, Vec2::new(0., 5.)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Knight, PieceColor::White, Vec2::new(0., 6.)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    Piece::new(PieceType::Rook, PieceColor::White, Vec2::new(0., 7.)).spawn(
        &mut commands,
        mesh.clone(),
        &mut materials,
        &piece_assets,
    );

    for i in 0..8 {
        Piece::new(PieceType::Pawn, PieceColor::White, Vec2::new(1., i as f32)).spawn(
            &mut commands,
            mesh.clone(),
            &mut materials,
            &piece_assets,
        );
    }
}
