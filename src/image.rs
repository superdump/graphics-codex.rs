use crate::color::{pixel_value, Radiance};
use crate::point::vector3;

use rayon::iter::IntoParallelRefMutIterator;
use rayon::prelude::*;
use rayon::slice::IterMut;

pub struct Image {
    pub width: usize,
    pub height: usize,
    data: Vec<Radiance>,
    data_argb8888: Vec<u8>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let mut image = Image {
            width,
            height,
            data: vec![vector3(0f32, 0f32, 0f32); width * height],
            data_argb8888: vec![0; width * height * 4],
        };

        for i in 0..(width * height) {
            image.data_argb8888[4 * i] = 255;
        }

        image
    }

    pub fn set(&mut self, x: usize, y: usize, l_i: Radiance) {
        // println!("{}:{} = {}:{}:{}", x, y, l_i.x, l_i.y, l_i.z);
        let index = y * self.width + x;
        self.data[index] = l_i;
    }

    pub fn as_argb8888(&mut self) -> &[u8] {
        self.data
            .par_iter()
            .zip(self.data_argb8888.par_chunks_mut(4))
            .for_each(|(l, argb)| {
                let color = pixel_value(*l, 1.0f32, 2.2f32) * 255.99f32;
                argb[0] = color.x as u8;
                argb[1] = color.y as u8;
                argb[2] = color.z as u8;
            });

        self.data_argb8888.as_slice()
    }
}

impl<'data> IntoParallelRefMutIterator<'data> for Image {
    type Item = &'data mut Radiance;
    type Iter = IterMut<'data, Radiance>;

    fn par_iter_mut(&'data mut self) -> Self::Iter {
        self.data.par_iter_mut()
    }
}

pub fn image(width: usize, height: usize) -> Image {
    Image::new(width, height)
}
