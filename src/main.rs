use anyhow::Result;
use image::{ImageBuffer, Rgba};
use std::{env, process::Command};
use x11rb::{connection::Connection, protocol::xproto::ConnectionExt};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let (conn, screen_num) = x11rb::connect(None)?;

    let screen = &conn.setup().roots[screen_num];

    let geometry = conn.get_geometry(screen.root)?.reply()?;

    let (image, width, height) = match args.get(1).map(String::as_str) {
        Some("--select") => {
            let (x, y, width, height) = select_region()?;

            let image = conn
                .get_image(
                    x11rb::protocol::xproto::ImageFormat::Z_PIXMAP,
                    screen.root,
                    x,
                    y,
                    width,
                    height,
                    u32::MAX,
                )?
                .reply()?;

            (image, width as u32, height as u32)
        }
        _ => {
            let width = geometry.width as u16;
            let height = geometry.height as u16;

            let image = conn
                .get_image(
                    x11rb::protocol::xproto::ImageFormat::Z_PIXMAP,
                    screen.root,
                    0,
                    0,
                    width,
                    height,
                    u32::MAX,
                )?
                .reply()?;

            (image, width as u32, height as u32)
        }
    };

    let buffer = image.data;

    let mut img = ImageBuffer::<Rgba<u8>, _>::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let i = ((y * width + x) * 4) as usize;

            let pixel = Rgba([
                buffer[i + 2], // red
                buffer[i + 1], // green
                buffer[i],     // blue
                255,
            ]);

            img.put_pixel(x, y, pixel);
        }
    }

    img.save("screenshot.png")?;

    println!("saved screenshot.png");

    Ok(())
}
fn select_region() -> Result<(i16, i16, u16, u16)> {
    let output = Command::new("slop").args(["-f", "%x %y %w %h"]).output()?;

    let coords = String::from_utf8(output.stdout)?;

    let mut values = coords.split_whitespace();

    let x = values.next().unwrap().parse()?;
    let y = values.next().unwrap().parse()?;
    let w = values.next().unwrap().parse()?;
    let h = values.next().unwrap().parse()?;

    Ok((x, y, w, h))
}
