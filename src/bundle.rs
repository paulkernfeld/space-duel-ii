use crate::systems::MoveShipsSystem;
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
        builder.add(MoveShipsSystem, "ship_system", &[]);
        Ok(())
    }
}
