use image;
use image::imageops::Lanczos3;

pub fn get_image(path: &str) -> Result<image::DynamicImage, image::ImageError> {

    let img = image::open(path)?;
    Ok(img)

}

pub fn resize(sizes: &Vec<u32>, image: &image::DynamicImage, name: &str) {

    for size in sizes {

        let file_name = format!("{0}_{1}_{1}.png", name, size);
        let im = image.resize_to_fill(*size, *size, Lanczos3);

        let status = match im.save(file_name) {
            Ok(_) => "succeed",
            Err(_) => "failed"
        };
        
        println!("{1} to create {0}x{0} image", size, status);
    }

}

