use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub const LIGHT_YELLOW: Color = Color::rgb(0.5, 0.6, 0.3);
pub const YELLOW: Color = Color::rgb(0.9, 0.9, 0.8);

pub const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|mat| StandardMaterial {
        base_color: Color::rgb(0.8, 0.3, 0.3),
        ..mat.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|mat| StandardMaterial {
        base_color: Color::rgb_u8(46, 37, 9),
        ..mat.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|mat| StandardMaterial {
        base_color: Color::rgb(0.9, 0.1, 0.1),
        ..mat.to_owned()
    })),
};
