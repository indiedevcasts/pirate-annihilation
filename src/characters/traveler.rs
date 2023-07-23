use bevy::{
    prelude::{info, Component, Entity, Query, Res, Transform, Vec3, With},
    time::Time,
};
use rand::seq::SliceRandom;

use crate::hex::{self, HexCell};

#[derive(Component, Default)]
pub struct Traveler {
    pub destination: Option<(f32, f32)>,
}

fn random_destination(hex_cells: Vec<&HexCell>) -> (f32, f32) {
    let random_cell: Option<&&HexCell> = hex_cells.choose(&mut rand::thread_rng());

    if let Some(random_cell) = random_cell {
        (random_cell.coordinates.0, random_cell.coordinates.1)
    } else {
        (0., 0.)
    }
}

// Simple implementation to move a traveler to a random cell. Later will use
// way more complex movement influenced by decisions, nodes weight, ...
pub fn travel(
    mut travelers: Query<(&mut Transform, &mut Traveler)>,
    hex_cells: Query<&HexCell>,
    time: Res<Time>,
) {
    for (mut transform, mut traveler) in travelers.iter_mut() {
        let traveler_position = (transform.translation.x, transform.translation.z);
        let hex_cells = hex_cells.iter().collect::<Vec<&HexCell>>();

        // Retrieve the traveler's destination if any or set a new one.
        // If the destination is reached, create a new one.
        let dest: (f32, f32) = match traveler.destination {
            Some(destination) if destination == traveler_position => random_destination(hex_cells),
            Some(destination) => destination,
            None => random_destination(hex_cells),
        };

        info!("Dest is: {:?}", dest);

        // Update traveler's target destination
        traveler.destination = Some(dest);

        // TODO: figure out how to get the conversion world/coords for hex cells
        let move_vec = Vec3::new(dest.0, 0., dest.1) - transform.translation;

        // let move_vec =
        //     transform.rotation * Vec3::new(dest.0, 0., dest.1).clamp_length_max(1.0) * 5.;

        info!("move: {}", move_vec);
        // move towards a cell coordinates
        let current_position = transform.translation;
        let new_position = current_position + move_vec * time.delta_seconds() * 2.;

        transform.translation = new_position;
    }
}
