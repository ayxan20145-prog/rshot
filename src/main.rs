use anyhow::Result;
use image::{ImageBuffer, Rgba};
use x11rb::{connection::Connection, protocol::xproto::ConnectionExt};

fn main() -> Result<()> {
    let (conn, screen_num) = x11rb::connect(None)?;

    let screen = &conn.setup().roots[screen_num];

    let geometry = conn.get_geometry(screen.root)?.reply()?;

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

    let width = width as u32;
    let height = height as u32;

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
