/*
** src/puzzles/day_20.rs
** https://adventofcode.com/2021/day/20
*/

use crate::types::{Puzzle, PuzzleError, Result, Solution};

const IMG_ENH_ALG_SIZE: usize = 512;
const INPUT_SIZE: usize = 100;

#[derive(Debug, Clone, Copy)]
enum Pixel {
    Dark,
    Light,
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Dark,
            '#' => Self::Light,
            _ => unreachable!(),
        }
    }
}

struct Algorithm {
    string: [Pixel; IMG_ENH_ALG_SIZE],
}

impl Algorithm {
    fn get(&self, n: u16) -> Pixel {
        self.string[n as usize]
    }
}

impl From<&'static str> for Algorithm {
    fn from(s: &'static str) -> Self {
        let mut string = [Pixel::Dark; IMG_ENH_ALG_SIZE];
        for (i, c) in s.chars().enumerate() {
            string[i] = Pixel::from(c);
        }

        Self { string }
    }
}

#[derive(Clone)]
struct Image {
    pixels: Vec<Vec<Pixel>>,
    size: usize,
}

impl Image {
    fn blank(size: usize) -> Self {
        let pixels = vec![vec![Pixel::Dark; size]; size];
        Self { pixels, size }
    }

    fn from_string(s: &'static str, size: usize) -> Self {
        let mut pixels = Vec::with_capacity(size);

        for row in s.split_whitespace() {
            let mut pixel_row = Vec::with_capacity(size);
            for c in row.chars() {
                pixel_row.push(Pixel::from(c));
            }
            pixels.push(pixel_row);
        }

        Self { pixels, size }
    }

    fn pad(&self, padding: usize) -> Self {
        let mut output = Self::blank(self.size + (padding * 2));
        for (i, j) in itertools::iproduct!(0..self.size, 0..self.size) {
            output.pixels[i + padding][j + padding] = self.pixels[i][j];
        }
        output
    }

    fn set(&mut self, i: usize, j: usize, pixel: Pixel) {
        self.pixels[i][j] = pixel;
    }

    fn lit_pixels(&self) -> usize {
        self.pixels
            .iter()
            .map(|row| row.iter().filter(|p| matches!(p, Pixel::Light)).count())
            .sum()
    }

    fn get_or(&self, i: usize, j: usize, di: i64, dj: i64, or: Pixel) -> Pixel {
        // passed as usize for better interface
        let i = i as i64;
        let j = j as i64;

        let i_in_range = i + di >= 0 && i + di < self.size as i64;
        let j_in_range = j + dj >= 0 && j + dj < self.size as i64;

        if i_in_range && j_in_range {
            self.pixels[(i + di) as usize][(j + dj) as usize]
        } else {
            or
        }
    }

    fn window(&self, i: usize, j: usize, default_pixel: Pixel) -> u16 {
        let mut n = 0;
        for (offset, (di, dj)) in itertools::enumerate(itertools::iproduct!(-1..=1, -1..=1)) {
            match self.get_or(i, j, di, dj, default_pixel) {
                Pixel::Light => n |= 1 << (8 - offset),
                _ => (),
            };
        }
        n
    }
}

pub struct Day20 {
    algorithm: Algorithm,
    image: Image,
}

impl Day20 {
    pub fn new(input: &'static str) -> Self {
        split_into!(input, "\n\n", alg_str, img_str);

        let algorithm = Algorithm::from(alg_str);
        let image = Image::from_string(img_str, INPUT_SIZE);

        Self { algorithm, image }
    }

    fn process_image_single_round(&self, image: Image, round: usize) -> Image {
        let mut output = Image::blank(image.size);
        // from observing the algorithm, an all-dark window results in a light
        // pixel and an all-light window results in a dark pixel, so alternate
        // between the two for the "infinite" region
        let default_pixel = if round % 2 == 0 {
            Pixel::Dark
        } else {
            Pixel::Light
        };

        for (i, j) in itertools::iproduct!(0..image.size, 0..image.size) {
            let index = image.window(i, j, default_pixel);
            output.set(i, j, self.algorithm.get(index));
        }

        output
    }

    fn process_image(&self, image: Image, n_rounds: usize) -> Image {
        let mut output = image;
        for round in 0..n_rounds {
            output = self.process_image_single_round(output, round);
        }
        output
    }
}

impl Puzzle for Day20 {
    // Start with the original input image and apply the image enhancement
    // algorithm twice, being careful to account for the infinite size of the
    // images. How many pixels are lit in the resulting image?
    fn part_1(&self) -> Result<Solution> {
        let n_rounds = 2;
        // add sufficient padding to simulate the "infinite" image
        let input = self.image.pad(n_rounds * 2);
        let output = self.process_image(input, n_rounds);
        Ok(output.lit_pixels().into())
    }

    // Start again with the original input image and apply the image
    // enhancement algorithm 50 times. How many pixels are lit in the
    // resulting image?
    fn part_2(&self) -> Result<Solution> {
        let n_rounds = 50;
        // add sufficient padding to simulate the "infinite" image
        let input = self.image.pad(n_rounds * 2);
        let output = self.process_image(input, n_rounds);
        Ok(output.lit_pixels().into())
    }
}
