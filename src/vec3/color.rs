use crate::Color;
use crate::rtweekend::clamp;

use std::fs::{File, OpenOptions};
use std::io::{Write, Error, BufWriter,copy};
use crate::vec3::Vec3;
use rayon::iter::{ParallelIterator, IntoParallelIterator, IntoParallelRefIterator};
use std::sync::Mutex;
use rayon::iter::Map;
use rayon::iter::*;
use image::{ImageFormat, ImageOutputFormat, DynamicImage, ImageBuffer};

pub fn write_color(img: Vec<Vec<Vec3>>, file: &str) -> Result<(), Error>{
    let sizey = img.len() as u32;
    let sizex = img[0].len() as u32;

    let mut imgbuf = image::ImageBuffer::new(sizex, sizey);
    let mut imgborrow = Mutex::new(imgbuf);
    img.par_iter().enumerate().for_each(|(y,row)|{

        row.par_iter().enumerate().for_each(|(x,pixel)|{
            (imgborrow.lock().unwrap()).put_pixel(x as u32, y as u32, image::Rgb([pixel.x() as u8, pixel.y() as u8, pixel.z() as u8]));




        })
    });
    let mut fet = imgborrow.lock().unwrap().clone();

    DynamicImage::ImageRgb8((fet)).save(file);

     // for (y, row) in img.iter().enumerate() {
     //     for (x, pixel) in row.iter().enumerate() {
     //         imgbuf.put_pixel(x as u32, y as u32, image::Rgb([pixel.x() as u8, pixel.y() as u8, pixel.z() as u8]));
     //     }};

    Ok(())




}


