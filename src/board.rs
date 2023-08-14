use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_board);
    }
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane::default()));
    let light_material = materials.add(Color::rgb_u8(124, 149, 93).into());
    let dark_material = materials.add(Color::rgb_u8(238, 238, 213).into());

    for i in 0..8 {
        for j in 0..8 {
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: if (i + j + 1) % 2 == 0 {
                        light_material.clone()
                    } else {
                        dark_material.clone()
                    },
                    transform: Transform::from_xyz(i as f32, 0., j as f32),
                    ..default()
                },
                PickableBundle::default(),
                RaycastPickTarget::default(),
            ));
        }
    }
}
