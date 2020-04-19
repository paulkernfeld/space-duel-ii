use crate::{Bullet, Ship, ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    core::math::Vector3,
    ecs::prelude::World,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use std::f32::consts::PI;

#[derive(Default)]
pub struct Game {
    ball_spawn_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        // Wait one second before spawning the ball.
        self.ball_spawn_timer.replace(1.0);

        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));
        initialize_ship(world, self.sprite_sheet_handle.clone().unwrap());
        initialize_bullet(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_camera(world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/ship.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/ship_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle), // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialize_ship(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut ship_transform = Transform::default();
    ship_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
    ship_transform.set_scale(Vector3::new(0.1f32, 0.1f32, 0.1f32));

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ship { dx: 0.0, dy: 0.0 })
        .with(ship_transform)
        .build();
}

fn initialize_bullet(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
    transform.set_rotation_2d(PI);
    transform.set_scale(Vector3::new(0.05f32, 0.05f32, 0.05f32));

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Bullet { dx: 0.0, dy: 0.0 })
        .with(transform)
        .build();
}
