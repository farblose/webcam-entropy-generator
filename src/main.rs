use nokhwa::{self};
use num_bigint::BigUint;
use num_traits::Num;
use std::env;

// Function to convert a binary string to a BigUint
fn binary_to_decimal_big(binary_str: &str) -> Result<BigUint, num_bigint::ParseBigIntError> {
    BigUint::from_str_radix(binary_str, 2)
}

// Function to calculate the number of bits in a camera frame
fn get_frame_bits() -> Result<u64, Box<dyn std::error::Error>> {
    let index = nokhwa::utils::CameraIndex::Index(0);
    let requested = nokhwa::utils::RequestedFormat::new::<nokhwa::pixel_format::LumaFormat>(
        nokhwa::utils::RequestedFormatType::AbsoluteHighestFrameRate,
    ); // Request the highest frame rate in Luma format
    let camera = nokhwa::Camera::new(index, requested)?; // Initialize the camera

    Ok(camera.resolution().x() as u64 * camera.resolution().y() as u64 + 1)
}

// Function to capture a specified number of frames from the camera
fn capture(n: u64) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    let index = nokhwa::utils::CameraIndex::Index(0);
    let requested = nokhwa::utils::RequestedFormat::new::<nokhwa::pixel_format::LumaFormat>(
        nokhwa::utils::RequestedFormatType::AbsoluteHighestFrameRate,
    ); // Request the highest frame rate in Luma format
    let mut camera = nokhwa::Camera::new(index, requested)?; // Initialize the camera

    camera.open_stream()?;

    // Warm up the webcam by capturing 25 frames
    for _ in 0..25 {
        camera.frame().ok();
    }

    let mut photos = Vec::with_capacity(n as usize);

    // Capture 'n' frames and decode them into vectors of bytes
    for _ in 0..n {
        let frame = camera.frame()?;
        let decoded = frame
            .decode_image::<nokhwa::pixel_format::LumaFormat>()?
            .into_vec();

        photos.push(decoded);
    }

    Ok(photos)
}

// Function to calculate entropy from captured frames and return a binary string
fn entropy(photos: Vec<Vec<u8>>, trim: u64) -> String {
    // Ensure all images have the same number of bytes
    if photos.is_empty() || photos.iter().any(|photo| photo.len() != photos[0].len()) {
        panic!("All images must have the same number of bytes!");
    }

    let mut skewed = String::with_capacity(photos[0].len());
    skewed.push('1');

    // Calculate the XOR of each pixel between consecutive frames
    for i in 1..photos.len() {
        for j in 0..photos[0].len() {
            let pixel1 = photos[i - 1][j];
            let pixel2 = photos[i][j];

            let xor_result = pixel1 ^ pixel2;

            skewed.push(if xor_result & 1 == 1 { '1' } else { '0' });
        }
    }

    // Trim the result to the specified length
    skewed.chars().take(trim as usize).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: ./weg [size of number in bits]");
    }

    let trim = args[1].parse::<u64>().expect("Invalid number format");
    let frame_size = get_frame_bits().expect("Error getting size of frame");
    let frames_needed: u64;
    if trim < frame_size {
        frames_needed = 2;
    } else {
        frames_needed = 2 + (trim - 1) / frame_size;
    }

    // Capture frames and calculate entropy
    match capture(frames_needed) {
        Ok(photos) => {
            let res = entropy(photos, trim);
            match binary_to_decimal_big(&res) {
                Ok(decimal) => println!("{}", decimal),
                Err(err) => eprintln!("Error converting binary to decimal: {}", err),
            }
        }
        Err(err) => eprintln!("Error capturing images: {}", err),
    }
}
