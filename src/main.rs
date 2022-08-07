use bmp::{px, Image, Pixel};
use itertools::Itertools;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::default::Default;
use std::fmt::Debug;


mod simulation;
mod rng;


const COOLING: u8 = 55;

type Heat = u8;

#[derive(Debug)]
struct Uniforms {
    decay: Uniform<u8>,
    spark: Uniform<u8>,
}

#[derive(Debug)]
struct Cells<const N: usize> {
    heat_map: [Heat; N],
    uniforms: Uniforms,
}

impl<const N: usize> Cells<N> {
    fn new() -> Self {
        Cells {
            heat_map: [Default::default(); N],
            uniforms: Uniforms {
                decay: Uniform::from(0..(((COOLING / 10) * N as u8) + 2)),
                spark: Uniform::from(160..255),
            },
        }
    }

    fn step(self) -> Cells<N> {
        let uniforms = self.uniforms;
        let mut next: [Heat; N] = [0; N];
        let mut rng = rand::thread_rng();

        // Heat diffuses from the first cell to the last
        // as the mean of the current cell and its previous
        // contents.
        //
        // First the entire system cools slightly
        // Next we iterate the heat_map twice, once with a
        // one-cell offset and calculate the mean of each
        // cell:
        //
        //     sliding:  [h1, h2, h3, h4, h5, h6, ...]
        //     heat_map: [h2, h3, h4, h5, h6, ...]
        //
        let (mut heat_map, sliding) = IntoIterator::into_iter(self.heat_map)
            .map(|cell| cell.saturating_sub(uniforms.decay.sample(&mut rng))) // Cool down every cell
            .tee();

        next[0] = heat_map.next().unwrap();
        heat_map
            .zip(sliding)
            .map(|(current, previous)| ((current as u16 + previous as u16) / 2) as u8)
            .enumerate()
            .for_each(|(index, heat)| next[index + 1] = heat);

        // Add a spark approximately every 1/N steps
        if rng.gen_ratio(1, N as u32) {
            let index = rng.gen_range(0..((N / 3) - 1));

            next[index] = next[index].saturating_add(uniforms.spark.sample(&mut rng));
        }

        // Dampen the flame 1/4 of the time.
        if rng.gen_ratio(1, 4) {
            // Dampen
            for cell in next.iter_mut() {
                *cell = cell.saturating_sub(uniforms.decay.sample(&mut rng));
            }
        }

        Cells {
            heat_map: next,
            uniforms,
        }
    }
}

const PALETTE: [&str; 4] = [" ", ".", "*", "#"];
impl<const N: usize> std::fmt::Display for Cells<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(write!(
            f,
            "{}",
            self.heat_map
                .iter()
                .map(|&c| PALETTE[(c / 64) as usize])
                .collect::<String>()
        )?)
    }
}

const BLACK: Pixel = px!(0, 0, 0);
const DARK_RED: Pixel = px!(140, 0, 25);
const RED: Pixel = px!(240, 0, 0);
const DARK_ORANGE: Pixel = px!(240, 130, 5);
const ORANGE: Pixel = px!(240, 160, 0);
const DARK_YELLOW: Pixel = px!(250, 180, 10);
const YELLOW: Pixel = px!(240, 240, 0);
const WHITE: Pixel = px!(255, 255, 255);

const COLOR_PALETTE: [Pixel; 8] = [
    BLACK,
    DARK_RED,
    RED,
    DARK_ORANGE,
    ORANGE,
    DARK_YELLOW,
    YELLOW,
    WHITE,
];

fn main() {
    let mut image = Image::new(1000, 60);

    let mut heat_map: Cells<6> = Cells::new();
    for x in 0..1000 {
        heat_map = heat_map.step();

        for (y, cell) in (&(heat_map.heat_map)).iter().enumerate() {
            image.set_pixel(x, y as u32, COLOR_PALETTE[(cell / 32) as usize]);
        }

        // TODO(DEBUG);
        println!("{:3}: {} ({:?})", x, heat_map, heat_map.heat_map);
    }

    let _ = image.save("test.bmp");
}
