use crate::{Ship, ARENA_WIDTH, ARENA_HEIGHT};
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
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

            let mut ship_x = local.translation().x;
            let mut ship_y = local.translation().y;

            if ship_x <= 0.0 {
                ship_x = -ship_x;
                ship.dx = -ship.dx;
            }
            if ship_y <= 0.0 {
                ship_y = -ship_y;
                ship.dy = -ship.dy;
            }
            if ship_x >= ARENA_WIDTH {
                ship_x = ARENA_WIDTH - (ship_x - ARENA_WIDTH);
                ship.dx = -ship.dx;
            }
            if ship_y >= ARENA_HEIGHT {
                ship_y = ARENA_HEIGHT - (ship_y - ARENA_HEIGHT);
                ship.dy = -ship.dy;
            }

            local.translation_mut().x = ship_x;
            local.translation_mut().y = ship_y;
        }
    }
}
