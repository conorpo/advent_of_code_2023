use std::path::Iter;
use std::vec;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

fn main() -> Result<(), Error> {
    let input = include_str!("./11.txt");
    run(input);
    Ok(());
}

fn run(input: &str) -> Result<(), Error> {
    let event_loop = EventLoop::new().unwrap();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut problem_visualizer = ProblemVisualizer::new(input);
    problem_visualizer.pre_comp();

    event_loop.set_control_flow(ControlFlow::Poll);

    let _ = event_loop.run(move |event, elwt| {
        match event {
            Event::AboutToWait => {
                problem_visualizer.update(input);
            },
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                problem_visualizer.draw(pixels.frame_mut());
                if pixels.render().is_err() {
                    eprintln!("pixels.render() failed");
                    elwt.exit();
                }
            },
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                elwt.exit();
            },
            _ => {}
        }
    });

    let part1_output = part1(input);
    dbg!(part1_output);
    let part2_output = part2(input);
    dbg!(part2_output);

    Ok(())
}

struct ProblemVisualizer<'a> {
    input: String,
    h: usize,
    w: usize,
    expanded_rows: Vec<bool>,
    expanded_cols: Vec<bool>,

    pair_vec: Vec<((usize, usize), (usize, usize))>,
    pair_iter: Box<dyn Iterator<Item = ((usize, usize), (usize, usize))> + 'a>,
}

impl ProblemVisualizer<'_> {
    fn new(input: &str) -> Self {
        let input = input.to_string();
        let mut temp_iter = input.lines();
        let w = temp_iter.next().unwrap().chars().count();
        let h = temp_iter.count() + 1;

        Self {
            input,
            h,
            w,
            expanded_rows: vec![true; h], // True so we can falsify when we find a star
            expanded_cols: vec![true; w],
            pair_vec: Vec::new(),
            pair_iter: Box::new(vec![].into_iter()),
        }
    }

    fn pre_comp(&self) {
        let mut star_coords = Vec::new();

        for (row, line) in self.input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                match c {
                    '.' => {},
                    '#' => {
                        star_coords.push((col, row));
                        self.expanded_rows[row] = false;
                        self.expanded_cols[col] = false;
                    }
                    _ => panic!("Invalid character in input"),
                }
            }
        }

        for pair_a in star_coords.iter() {
            for pair_b in star_coords.iter() {
                if pair_a != pair_b {
                    self.pair_vec.push((*pair_a, *pair_b));
                }
            }
        }

        self.pair_iter = Box::new(self.pair_vec.into_iter());
    }

    fn update(&mut self, _input: &str) {
        if let Some((pair_a, pair_b)) = self.pair_iter.next() {
            // Do something
        }
    }

    fn draw(&mut self, pixels: &mut [u8]) {
        for (i, pixel) in pixels.chunks_exact_mut(4).enumerate() {
            let x = i % WIDTH as usize;
            let y = i / WIDTH as usize;

            let color = if x % 2 == 0 && y % 2 == 0 {
                [0, 0, 0, 255]
            } else {
                [255, 255, 255, 255]
            };

            pixel.copy_from_slice(&color);
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;


    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const test_input: &str = 
"";

    #[test]
    fn test_part1() {
        assert_eq!(part1(test_input), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(test_input), 0);
    }
}
