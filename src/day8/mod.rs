use std::fs::File;
use std::io::prelude::*;

pub fn solve() {

    let input = include_str!("./input.txt");
    let width = 25;
    let height = 6;
    let data: Vec<u8> = input.chars().map(|x| x.to_digit(10).unwrap() as u8).collect();
    let layer = layer_with_fewest_zeros(&data, width, height);

    let image = pixels(&data, width, height, layer);
    let ones = count_pixels(image, |x| { x == 1 });
    let twos = count_pixels(image, |x| { x == 2 });
    let sum = ones * twos;

    println!("{:?}", sum);

    let size = width * height;
    let layers = data.len() / size;

    let mut flattened_image = vec![2; size];

    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            for layer in 0..layers {
                let index_src = layer * size + index;
                if flattened_image[index] == 2 {
                    flattened_image[index] = data[index_src];
                }
            }
        }
    }

    print_image(&flattened_image, width, height);
    write_ppm(&image, width, height).unwrap();

}

fn print_image(image: &[u8], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let pixel = image[index];
            match pixel {
                1 => print!("\u{25a0}"),
                _ => print!(" ")
            }
        }
        println!();
    }
}

fn write_ppm(image: &[u8], width: usize, height: usize) -> Result<(), std::io::Error> {
    let mut file = File::create("day8.ppm").unwrap();
    write!(file, "P6\n")?;
    write!(file, "{}\n", width)?;
    write!(file, "{}\n", height)?;
    write!(file, "{}\n", 255)?;
    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let pixel = image[index];
            let pixel = match pixel {
                1 => 255,
                _ => 0
            };
            file.write(&[pixel, pixel, pixel])?;
        }
    }
    Ok(())
}

fn pixels(image: &[u8], width: usize, height: usize, layer: usize) -> &[u8] {
    let start = layer * (width * height);
    let end = start + (width * height);
    return &image[start..end];
}

fn count_pixels<F>(image: &[u8], predicate: F) -> usize 
    where F: Fn(u8) -> bool { 
    return image.iter().filter(|x| predicate(**x)).count()
}

fn layer_with_fewest_zeros(image: &[u8], width: usize, height: usize) -> usize {
    let layers = image.len() / (width * height);
    let range = 0..layers-1;
    let min_zeros = range
        .enumerate()
        .map(|(index, layer)| (index, count_pixels(pixels(image, width, height, layer), |pixel| pixel == 0)))
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    min_zeros.0
}