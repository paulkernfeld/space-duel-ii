use crate::Ship;
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
#[derive(SystemDesc)]
pub struct MoveShipsSystem;

impl<'s> System<'s> for MoveShipsSystem {
    type SystemData = (
        WriteStorage<'s, Ship>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut ships, mut locals, time): Self::SystemData) {
        for (mut ship, local) in (&mut ships, &mut locals).join() {
            ship.dx += 1.0 * time.delta_seconds();
            ship.dy += 1.0 * time.delta_seconds();
            local.prepend_translation_x(ship.dx * time.delta_seconds());
            local.prepend_translation_y(ship.dy * time.delta_seconds());
        }
    }
}
