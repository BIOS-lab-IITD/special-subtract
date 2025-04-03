use image::{DynamicImage, GenericImageView, ImageBuffer, ImageError, ImageReader, Rgba};
// use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{fs, io}; //filesystem related

fn img_subtract(img1: &DynamicImage, img2: DynamicImage) -> DynamicImage {
    if img1.dimensions() != img2.dimensions() {
        panic!("Fatal Error: Image dimensions minmatched!\nQuitting the program!")
    }

    let (width, height) = img1.dimensions();
    let mut output_img = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let pixel1 = img1.get_pixel(x, y);
            let pixel2 = img2.get_pixel(x, y);

            // Invert pixel2 colors
            let inverted_r = 255 - pixel2[0];
            let inverted_g = 255 - pixel2[1];
            let inverted_b = 255 - pixel2[2];
            let inverted_a = pixel2[3]; // Alpha remains the same.

            // 50% opacity blending (averaging)
            let blended_r = ((pixel1[0] as u16 + inverted_r as u16) / 2) as u8;
            let blended_g = ((pixel1[1] as u16 + inverted_g as u16) / 2) as u8;
            let blended_b = ((pixel1[2] as u16 + inverted_b as u16) / 2) as u8;
            let blended_a = ((pixel1[3] as u16 + inverted_a as u16) / 2) as u8;

            output_img.put_pixel(x, y, Rgba([blended_r, blended_g, blended_b, blended_a]));
        }
    }

    DynamicImage::ImageRgba8(output_img) //being returned post subtraction
}

fn process_images_in_folder(folder_path: &str, output_folder: &str) -> Result<(), ImageError> {
    fs::create_dir_all(output_folder).unwrap(); // create output folder if it doesnt exist.

    let paths = fs::read_dir(folder_path).unwrap();
    let image_paths: Vec<_> = paths
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if let Some(ext_str) = ext.to_str() {
                        if ["jpg", "jpeg", "png", "bmp", "gif"]
                            .contains(&ext_str.to_lowercase().as_str())
                        {
                            return Some(path);
                        }
                    }
                }
            }
            None
        })
        .collect();

    if image_paths.len() < 2 {
        println!("At least two images are needed for finding difference.");
        return Ok(());
    } else {
        println!(
            "Found a total of {} images in the input path",
            image_paths.len()
        );
    }

    let mut write_iter = 0; //file write iteration number
    for window in image_paths.windows(2) {
        let img1 = ImageReader::open(&window[0])?.decode()?;
        let img2 = ImageReader::open(&window[1])?.decode()?;

        let subtracted_img = img_subtract(&img1, img2);

        let output_filename = format!(
            "{}/diff_{}_{}.png",
            output_folder,
            window[0].file_stem().unwrap().to_str().unwrap(),
            window[1].file_stem().unwrap().to_str().unwrap()
        );

        subtracted_img.save(output_filename)?;
        write_iter += 1;
        println!(
            "Saved: difference image {} out of {}",
            write_iter,
            image_paths.len() - 1
        );
    }

    Ok(())
}

fn main() -> Result<(), ImageError> {
    //Get folder paths from user
    let mut input_folder_path = String::new();
    let mut output_folder_path = String::new();
    println!("Enter input folder path:");

    io::stdin()
        .read_line(&mut input_folder_path)
        .expect("Failed in reading input folder path");
    let input_folder_path = input_folder_path.trim();

    println!("Enter output folder path:");

    io::stdin()
        .read_line(&mut output_folder_path)
        .expect("Failed in reading output folder path");

    let output_folder_path = output_folder_path.trim();

    // Hard-coded path - to be removed!
    // let input_folder_path = r#"E:\rust\special_subtract\data_in"#; // Replace with your input folder
    // let output_folder_path = r#"E:\rust\special_subtract\data_o"#; // Replace with your output folder.

    let start = Instant::now();

    process_images_in_folder(&input_folder_path, &output_folder_path)?;
    let elapsed_time = start.elapsed();
    println!("Total time elapsed: {:?}", elapsed_time);

    println!("{}", "-".repeat(90));
    println!(
        "Thank you for using this piece of code 😃. You may provide your feedback at kmanish@iitd.ac.in"
    );
    println!(
        "Copyright © 2025 Manish Kumar, BIOS Lab, Indian Institute of Technology Delhi (India)"
    );
    println!("{}", "-".repeat(90));
    Ok(())
}
