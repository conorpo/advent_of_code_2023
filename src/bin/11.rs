use std::ops::RangeInclusive;
use std::path::Iter;
use std::vec;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder};

const WIDTH: u32 = 140;
const HEIGHT: u32 = 140;

fn main() -> Result<(), Error> {
    let input = include_str!("./11.txt");
    run(input)?;
    Ok(())
}

fn run(input: &str) -> Result<(), Error> {
    let event_loop = EventLoop::new().unwrap();

    let window = {
        let size = LogicalSize::new((WIDTH*4) as f64, (HEIGHT*4) as f64);
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
    problem_visualizer.pre_comp(pixels.frame_mut());

    event_loop.set_control_flow(ControlFlow::Poll);
    let mut done = false;

    let _ = event_loop.run(move |event, elwt| {
        match event {
            Event::AboutToWait => {
                if !done {
                    if pixels.render().is_err() {
                        eprintln!("pixels.render() failed");
                        elwt.exit();
                    }
                    done = problem_visualizer.update(input, pixels.frame_mut());
                }
            },
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                elwt.exit();
            },
            _ => {}
        }
    });

    Ok(())
}

struct ProblemVisualizer {
    input: String,
    h: usize,
    w: usize,
    expanded_rows: Vec<bool>,
    expanded_cols: Vec<bool>,

    pair_iter: Box<dyn Iterator<Item=((usize, usize), (usize, usize))>>,

    sum: u64,
}

impl ProblemVisualizer {
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
            pair_iter: Box::new(vec![].into_iter()),
            sum: 0,
        }
    }

    fn pre_comp(&mut self, frame: &mut [u8]) {
        let mut star_coords = Vec::new();

        // Set screen to black, alpha 1
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0, 0, 0, 255]); // [R, G, B, A]
        }

        for (row, line) in self.input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                match c {
                    '.' => {},
                    '#' => {
                        star_coords.push((col, row));
                        self.expanded_rows[row] = false;
                        self.expanded_cols[col] = false;

                        // Draw star
                        let index = (row * self.w + col) * 4;
                        frame[index..(index + 4)].copy_from_slice(&[255, 255, 255, 255]); // [R, G, B, A                        
                    }
                    _ => panic!("Invalid character in input"),
                }
            }
        }

        let mut star_pairs = Vec::new();
        let mut star_coords = star_coords.into_iter();
        while let Some(pair_a) = star_coords.next() {
            for pair_b in star_coords.clone() {
                if pair_a != pair_b {
                    star_pairs.push((pair_a, pair_b));
                }
            }
        }

        self.pair_iter = Box::new(star_pairs.into_iter());
    }

    fn intensify_pixel(&self, pixel: &mut [u8], channel: usize, intensity: f32) {
        let value = pixel[channel] as f32;
        let new_value = ((value * intensity).ceil() as u8).min(255).max(20);
        pixel[channel] = new_value;
    }

    fn plot_line(&self, pair_a: (usize, usize), pair_b: (usize, usize), pixels: &mut [u8], high: bool) {
        let (x1, y1) = (pair_a.0 as i32, pair_a.1 as i32);
        let (x2, y2) = (pair_b.0 as i32, pair_b.1 as i32);

        let mut dx = x2 - x1;
        let mut dy = y2 - y1;

        let (mut a,mut b,a2,b2,da,db) = match high {
            false => (x1, y1, x2, y2, &mut dx, &mut dy),
            true => (y1, x1, y2, x2, &mut dy, &mut dx),
        };

        let incrementor = (*db).signum();
        *db = (*db).abs();

        let mut d = 2 * (*db) - (*da);
        let distance_from_line = 0f32;
        let length = f32::sqrt(((*da)*(*da) + (*db)*(*db)) as f32);

        let sin = ((*db) as f32) / length;
        let cos = ((*da) as f32) / length;

        let range:RangeInclusive<i32> = -1..=1;

        while a <= a2 {
            for i in range.clone() {
                if (b+i) < 0 || (b+i) >= match high {
                    false => self.h as i32,
                    true => self.w as i32,
                } {
                    continue;
                }

                let index = match high {
                    false => ((b + i) as usize * self.w + a as usize) * 4,
                    true => (a as usize * self.w + (b + i) as usize) * 4,
                };

                let pixel = &mut pixels[index..(index + 4)];

                let channel = match high {
                    false => match (self.expanded_rows[(b+i) as usize], self.expanded_cols[a as usize]) {
                        (false, false) => 2,
                        _ => 0,
                    },
                    true => match (self.expanded_rows[a as usize], self.expanded_cols[(b+i) as usize]) {
                        (false, false) => 2,
                        _ => 0,
                    },
                };

                if match high {
                    false => (self.expanded_rows[(b+i) as usize], self.expanded_cols[a as usize]),
                    true => (self.expanded_rows[a as usize], self.expanded_cols[(b+i) as usize])
                } == (false, false) {
                    self.intensify_pixel(pixel, 2, 1.001);
                } else {
                    self.intensify_pixel(pixel, 0, 1.001);
                    self.intensify_pixel(pixel, 2, 1.001);
                }
            }

            a += 1;

            if d <= 0 {
                //distance_from_line += sin;
                d += 2 * (*db);
            } else {
                //distance_from_line += sin - cos;
                d += 2 * ((*db) - (*da));
                b += incrementor;
            }
        }

    }

    fn update(&mut self, _input: &str, pixels: &mut [u8]) -> bool {
        for _ in 1..300 {
            if let Some((pair_a, pair_b)) = self.pair_iter.next() {
                let high = (pair_b.1 as i32 - pair_a.1 as i32).abs() > (pair_b.0 as i32 - pair_a.0 as i32).abs();
                let (pair_tl, pair_br) = match match high {
                    false => pair_a.0 <= pair_b.0,
                    true => pair_a.1 <= pair_b.1,
                } {
                    true => (pair_a, pair_b),
                    false => (pair_b, pair_a),
                };

                self.plot_line(pair_tl, pair_br, pixels, high);

                //Summing
                let (t, b) = match pair_a.1 <= pair_b.1 {
                    true => (pair_a.1, pair_b.1),
                    false => (pair_b.1, pair_a.1),
                };

                let (l, r) = match pair_a.0 <= pair_b.0 {
                    true => (pair_a.0, pair_b.0),
                    false => (pair_b.0, pair_a.0),
                };

                let mut line_distance = 0u64;
                
                line_distance += self.expanded_cols[l..r].iter().fold(0u64, |acc, x| acc + (*x as u64)*999999 + 1);
                line_distance += self.expanded_rows[t..b].iter().fold(0u64, |acc, x| acc + (*x as u64)*999999 + 1);

                self.sum += line_distance;
            } else {
                dbg!(self.sum);
                return true;
            }
        }

        false
    }
}