use std::collections::HashMap;

use winit::event_loop::EventLoop;
use winit::event::{Event, WindowEvent, ElementState};
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;

use winit_input_helper::WinitInputHelper;

const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 480;

const MAP: [[u32; MAP_WIDTH]; MAP_HEIGHT] =
[
  [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
  [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
  [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
];

const WALL_HEIGHT: i32 = 20;

type PixelBuf = [u32; SCREEN_WIDTH * SCREEN_HEIGHT];

struct Vec2<T> {
    x: T,
    y: T
}

impl<T> Vec2<T> {
    fn new(x: T, y: T) -> Self {

        Self {
            x: x,
            y: y
        }

    }
}

struct Player {
    pos: Vec2<f64>,
    dir: Vec2<f64>,
    plane: Vec2<f64>
}

fn main() {

    let mut screen: PixelBuf = [0; SCREEN_HEIGHT * SCREEN_WIDTH];

    // initialise player structure
    let mut player = Player {
        pos: Vec2::<f64>::new(22.0, 12.0),
        dir: Vec2::<f64>::new(-1.0, 0.0),
        plane: Vec2::<f64>::new(0.0, 0.66)
    };

    // time for fps counter
    let mut time: f64 = 0.0;
    let mut old_time: f64 = 0.0;

    // setup winit
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Raycast Renderer")
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32))
        .with_resizable(false)
        .build(&event_loop)
        .expect("Unable to create window.");

    let input = WinitInputHelper::new();

    event_loop.run(move |event, _, control_flow| {

        control_flow.set_poll();

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {

                // todo input
                update(&mut screen, &mut player);

            },
            _ => ()
        }

    })

}

// TODO render with pixels and winit
fn render(framebuffer: &PixelBuf) {}

fn update(screen: &mut PixelBuf, player: &mut Player) {

    let WALL_COLOURS: HashMap<&str, (u8, u8, u8)> = HashMap::from([
        ("Red", (255, 0, 0)),
        ("Green", (0, 255, 0)),
        ("Blue", (0, 0, 255)), 
        ("White", (255, 255, 255)),
        ("Teal", (159, 252, 253)),
        ("Yellow", (255, 253, 85))
    ]);

    for x in 0..SCREEN_WIDTH {

        let camera_x: f64 = 2.0 * x as f64 / SCREEN_WIDTH as f64;
        let ray_dir = Vec2::<f64>::new(player.dir.x + player.plane.x * camera_x, player.dir.y + player.plane.y * camera_x);
        //let ray_dir_x: f64 = player.dir.x + player.plane.x * camera_x;

        // DDA setup
        let mut map_pos = Vec2::<i32>::new(player.pos.x.floor() as i32, player.pos.y.floor() as i32);
        let mut side_dist = Vec2::<f64>::new(0.0, 0.0);

        let delta_dist = Vec2::<f64>::new(if ray_dir.x == 0.0 { f64::INFINITY } else { (1.0 / ray_dir.x).abs() }, if ray_dir.y == 0.0 { f64::INFINITY } else { (1.0 / ray_dir.y).abs() });
        let mut perp_wall_dist: f64;

        let mut step = Vec2::<i32>::new(0, 0);
        
        let mut hit: bool = false;
        let mut side: u8 = 0;

        // get initial step and side dist values
        if ray_dir.x < 0.0 {
            step.x = -1;
            side_dist.x = (player.pos.x - map_pos.x as f64) * delta_dist.x;
        } else {
            step.x = 1;
            side_dist.x = (map_pos.x as f64 + 1.0 - player.pos.x) * delta_dist.x;
        }

        if ray_dir.y < 0.0 {
            step.y = -1;
            side_dist.y = (player.pos.y - map_pos.y as f64) * delta_dist.y;
        } else {
            step.y = 1;
            side_dist.y = (map_pos.y as f64 + 1.0 - player.pos.y) * delta_dist.y;
        }

        // DDA raycast
        while !hit {

            if side_dist.x < side_dist.y {
                side_dist.x += delta_dist.x;
                map_pos.x += step.x;
                side = 0;
            } else {
                side_dist.y += delta_dist.y;
                map_pos.y += step.y;
                side = 1;
            }

            if MAP[map_pos.x as usize][map_pos.y as usize] > 0 {
                hit = true;
            }

        }

        if side == 0 {
            perp_wall_dist = side_dist.x - delta_dist.x;
        } else {
            perp_wall_dist = side_dist.y - delta_dist.y;
        }

        let line_height: i32 = WALL_HEIGHT / perp_wall_dist as i32;

        let mut draw_start: i32 = -line_height / 2 + WALL_HEIGHT / 2;
        let mut draw_end: i32 = line_height / 2 + WALL_HEIGHT / 2;

        if draw_start < 0 {
            draw_start = 0
        }

        if draw_end > WALL_HEIGHT {
            draw_end = WALL_HEIGHT - 1;
        }

        let wall_colour_unpacked = match MAP[map_pos.x as usize][map_pos.y as usize] {
            1 => WALL_COLOURS.get(&"Red"),
            2 => WALL_COLOURS.get(&"Green"),
            3 => WALL_COLOURS.get(&"Blue"),
            4 => WALL_COLOURS.get(&"Teal"),
            _ => WALL_COLOURS.get(&"White")
        }.unwrap();

        let mut wall_colour = pack_colour(wall_colour_unpacked.0, wall_colour_unpacked.1, wall_colour_unpacked.2);

        // give walls a different brightness
        if side == 1 {
            wall_colour /= 2;
        }

        draw_line(screen, x, draw_start as usize, draw_end as usize, wall_colour);

    }

}

fn draw_line(framebuffer: &mut PixelBuf, x: usize, y0: usize, y1: usize, colour: u32) {

    for y in y0..y1 {
        framebuffer[x + y * SCREEN_WIDTH] = colour;
    }

}

// TODO
fn pack_colour(r: u8, g: u8, b: u8) -> u32 {
    ((0 as u32) << 24) + ((b as u32) << 16) + ((g as u32) << 8) + (r as u32)
}

fn unpack_color(color: &u32) -> (u8, u8, u8, u8) {
    let r: u8 = (color & 255) as u8;
    let g: u8 = ((color >> 8) & 255) as u8;
    let b: u8 = ((color >> 16) & 255) as u8;
    let a: u8 = ((color >> 24) & 255) as u8;

    (r, g, b, a)
}