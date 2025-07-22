use crate::engine::renderer::material::Texture;

pub fn load_texture_from_file(path: &str, name: String) -> Result<Texture, Box<dyn std::error::Error>> {
    let img = image::open(path)?;

    // Flip the image vertically so that its 0,0 coordinate is at the bottom left. 
    // Convert to RGBA8 format (4 bytes per pixel)
    let rgba = img.flipv().to_rgba8();

    let width = rgba.width();
    let height = rgba.height();
    
    let data: Vec<u8> = rgba.into_raw();

    Ok(Texture {
        name,
        width,
        height,
        data,
    })
}
