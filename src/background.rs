use bevy::prelude::*;
use image::{GenericImage, Rgba, RgbaImage};
// use noisy_bevy::{simplex_noise_2d, NoisyShaderPlugin};

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(build_background);
    }
}

const X_SIZE: u32 = 512;
const VERTICAL_COUNT: u32 = 4;

fn build_background(mut cmds: Commands, mut imgs: ResMut<Assets<Image>>) {
    let bg_colors = [
        Color::hex("#0c0d0c").unwrap(),
        Color::hex("#171a17").unwrap(),
        Color::hex("#212622").unwrap(),
        Color::hex("#29332c").unwrap(),
        Color::hex("#304036").unwrap(),
    ];

    let mut bg = RgbaImage::new(X_SIZE, X_SIZE * VERTICAL_COUNT);
    for bg_y in 0..VERTICAL_COUNT {
        let mut part = RgbaImage::new(X_SIZE, X_SIZE);
        for x in 0..X_SIZE {
            for y in 0..X_SIZE {
                let idx = fastrand::usize(..bg_colors.len());
                let col = bg_colors[idx];
                // let col = Color::hsla(0.0, fastrand::f32(), fastrand::f32().min(0.3), 1.0);

                let rgba_u8 = Rgba([
                    (col.r() * 255.0).round() as u8,
                    (col.g() * 255.0).round() as u8,
                    (col.b() * 255.0).round() as u8,
                    (col.a() * 255.0).round() as u8,
                ]);

                part.put_pixel(x, y, rgba_u8);
            }
        }

        bg.copy_from(&part, 0, bg_y * X_SIZE).expect("Zamn");
    }

    let bevy_image = Image::from_dynamic(bg.into(), true);
    let bg_handle = imgs.add(bevy_image);

    cmds.spawn(SpriteBundle {
        texture: bg_handle,
        ..default()
    });
}
