use crate::{Ship, ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

const SHIP_ENGINE_POWER: f32 = 4.0;
const SHIP_RUDDER_POWER: f32 = 2.0;

#[derive(SystemDesc)]
pub struct MoveShipsSystem;

impl<'s> System<'s> for MoveShipsSystem {
    type SystemData = (
        WriteStorage<'s, Ship>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut ships, mut locals, time, input): Self::SystemData) {
        for (mut ship, transform) in (&mut ships, &mut locals).join() {
            let rudder = input.axis_value("ship_rudder").unwrap();
            let engine_acceleration = input.axis_value("ship_engine").unwrap();

            let local: &mut Transform = transform;

            let (_, _, ship_angle) = local.euler_angles();

            ship.dx += ship_angle.cos() * engine_acceleration * time.delta_seconds() * SHIP_ENGINE_POWER;
            ship.dy += ship_angle.sin() * engine_acceleration * time.delta_seconds() * SHIP_ENGINE_POWER;
            local.prepend_translation_x(ship.dx * time.delta_seconds());
            local.prepend_translation_y(ship.dy * time.delta_seconds());
            local.prepend_rotation_z_axis(rudder * time.delta_seconds() * SHIP_RUDDER_POWER);

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
