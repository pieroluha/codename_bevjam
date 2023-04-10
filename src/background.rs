use crate::{GameState, InitNewStatus};
use bevy::prelude::*;
use image::{Rgba, RgbaImage};

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(build_background.in_schedule(OnExit(GameState::Startup)));
    }
}

const TEXTURE_SIDE: u32 = 2048;

fn build_background(mut cmds: Commands, mut imgs: ResMut<Assets<Image>>) {
    let bg_colors = [
        Color::hex("#0c0d0c").unwrap(),
        Color::hex("#171a17").unwrap(),
        Color::hex("#212622").unwrap(),
        Color::hex("#29332c").unwrap(),
        Color::hex("#304036").unwrap(),
    ];

    let mut bg_rgba = RgbaImage::new(TEXTURE_SIDE, TEXTURE_SIDE);

    for x in 0..TEXTURE_SIDE {
        for y in 0..TEXTURE_SIDE {
            let idx = fastrand::usize(..bg_colors.len());
            let col = bg_colors[idx];

            let pixel = Rgba([
                (col.r() * 255.0).round() as u8,
                (col.g() * 255.0).round() as u8,
                (col.b() * 255.0).round() as u8,
                (col.a() * 255.0).round() as u8,
            ]);

            bg_rgba.put_pixel(x, y, pixel);
        }
    }

    let bevy_image = bevy::prelude::Image::from_dynamic(bg_rgba.into(), true);
    let bg_handle = imgs.add(bevy_image);

    cmds.spawn(SpriteBundle {
        texture: bg_handle,
        ..default()
    });
}
