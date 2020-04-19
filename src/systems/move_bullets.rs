use crate::{Bullet, ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Entities, Join as _, Read, System, SystemData, WriteStorage},
};

const BULLET_SPEED: f32 = 10.0;

#[derive(SystemDesc)]
pub struct MoveBulletsSystem;

impl<'s> System<'s> for MoveBulletsSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Bullet>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut bullets, mut transforms, time): Self::SystemData) {
        for (entity, _bullet, transform) in (&*entities, &mut bullets, &mut transforms).join() {
            let local: &mut Transform = transform;

            let (_, _, bullet_angle) = local.euler_angles();

            let bullet_dx = bullet_angle.cos() * BULLET_SPEED;
            let bullet_dy = bullet_angle.sin() * BULLET_SPEED;
            local.prepend_translation_x(bullet_dx * time.delta_seconds());
            local.prepend_translation_y(bullet_dy * time.delta_seconds());

            let bullet_x = local.translation().x;
            let bullet_y = local.translation().y;

            if bullet_x < 0.0 || bullet_y < 0.0 || bullet_x > ARENA_WIDTH || bullet_y > ARENA_HEIGHT
            {
                entities.delete(entity).unwrap();
            }
        }
    }
}
