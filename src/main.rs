mod bundle;
mod game;
mod systems;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    ecs::{Component, DenseVecStorage},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use crate::bundle::SpaceDuelBundle;
use std::time::Duration;

const ARENA_HEIGHT: f32 = 100.0;
const ARENA_WIDTH: f32 = 100.0;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    use crate::game::Game;

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config/display.ron");

    let key_bindings_path = app_root.join("config/input.ron");
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
        )?
        .with_bundle(SpaceDuelBundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and
                // drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([1.0, 0.5, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::build(assets_dir, Game::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;

    game.run();
    Ok(())
}

#[derive(Default)]
pub struct Ship {
    pub dx: f32,
    pub dy: f32,
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Bullet {
    pub dx: f32,
    pub dy: f32,
}

impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}
