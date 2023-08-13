use bevy::prelude::*;

use super::component::{PieceColor, PieceType};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct PieceAssets {
    // black
    #[asset(path = "images/bb.png")]
    pub bb: Handle<Image>,
    #[asset(path = "images/bk.png")]
    pub bk: Handle<Image>,
    #[asset(path = "images/bn.png")]
    pub bn: Handle<Image>,
    #[asset(path = "images/bp.png")]
    pub bp: Handle<Image>,
    #[asset(path = "images/bq.png")]
    pub bq: Handle<Image>,
    #[asset(path = "images/br.png")]
    pub br: Handle<Image>,

    // white
    #[asset(path = "images/wb.png")]
    pub wb: Handle<Image>,
    #[asset(path = "images/wk.png")]
    pub wk: Handle<Image>,
    #[asset(path = "images/wn.png")]
    pub wn: Handle<Image>,
    #[asset(path = "images/wp.png")]
    pub wp: Handle<Image>,
    #[asset(path = "images/wq.png")]
    pub wq: Handle<Image>,
    #[asset(path = "images/wr.png")]
    pub wr: Handle<Image>,
}

impl PieceAssets {
    /// Return `Piece` texture correspondings to its type and color value.
    pub fn texture_from(&self, piece_type: PieceType, piece_color: PieceColor) -> Handle<Image> {
        match piece_color {
            PieceColor::White => match piece_type {
                PieceType::Rook => self.wr.clone(),
                PieceType::King => self.wk.clone(),
                PieceType::Pawn => self.wp.clone(),
                PieceType::Queen => self.wq.clone(),
                PieceType::Bishop => self.wb.clone(),
                PieceType::Knight => self.wn.clone(),
            },
            PieceColor::Black => match piece_type {
                PieceType::Rook => self.br.clone(),
                PieceType::King => self.bk.clone(),
                PieceType::Pawn => self.bp.clone(),
                PieceType::Queen => self.bq.clone(),
                PieceType::Bishop => self.bb.clone(),
                PieceType::Knight => self.bn.clone(),
            },
        }
    }
}
