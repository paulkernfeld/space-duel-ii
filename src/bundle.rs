use crate::systems::{MoveBulletsSystem, MoveShipsSystem};
use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};

pub struct SpaceDuelBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SpaceDuelBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(MoveShipsSystem, "move_ships_system", &[]);
        builder.add(MoveBulletsSystem, "move_bullet_system", &[]);
        Ok(())
    }
}
