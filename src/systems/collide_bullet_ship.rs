use crate::{Bullet, SHIP_RADIUS, Ship};
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Entities, Join as _, ReadStorage, System, SystemData},
};

#[derive(SystemDesc)]
pub struct CollideBulletShipSystem;

impl<'s> System<'s> for CollideBulletShipSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Bullet>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, bullets, ships, transforms): Self::SystemData) {
        for (bullet_entity, _bullet, bullet_transform) in (&*entities, &bullets, &transforms).join() {
            let bullet_x = bullet_transform.translation().x;
            let bullet_y = bullet_transform.translation().y;

            for (_ship, ship_transform) in (&ships, &transforms).join() {
                let ship_x = ship_transform.translation().x;
                let ship_y = ship_transform.translation().y;

                if (bullet_x - ship_x).powi(2) + (bullet_y - ship_y).powi(2) < SHIP_RADIUS.powi(2)
                {
                    entities.delete(bullet_entity).unwrap();
                }
            }
        }
    }
}
