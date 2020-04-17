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
        ReadStorage<'s, Ship>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, mut locals, time): Self::SystemData) {
        for (ship, local) in (&balls, &mut locals).join() {
            // local.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            // local.prepend_translation_y(ball.velocity[1] * time.delta_seconds());
            local.prepend_translation_x(time.delta_seconds());
            local.prepend_translation_y(time.delta_seconds());
        }
    }
}
