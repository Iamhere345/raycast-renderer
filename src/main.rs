use std::collections::HashMap;
use std::time::Instant;

use winit::event_loop::EventLoop;
use winit::event::{Event, WindowEvent, ElementState};
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;

use winit_input_helper::WinitInputHelper;

use pixels::{Error, Pixels, SurfaceTexture};

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

const MOVE_SPEED: f64 = 5.0;
const ROT_SPEED: f64 = 3.0;

type PixelBuf = Vec<u32>;

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
    
    println!("please");

    // this array is too big and causes a stack overflow
    let mut screen: PixelBuf = Vec::with_capacity(SCREEN_HEIGHT * SCREEN_WIDTH);
    unsafe { screen.set_len(SCREEN_HEIGHT * SCREEN_WIDTH) };

    // initialise player structure
    let mut player = Player {
        pos: Vec2::<f64>::new(22.0, 12.0),
        dir: Vec2::<f64>::new(-1.0, 0.0),
        plane: Vec2::<f64>::new(0.0, 0.66)
    };

    // time for fps counter
    let mut time: Instant = Instant::now();
    let mut old_time: Instant = Instant::now();

    // setup winit
    let event_loop = EventLoop::new();

    println!("before window");

    let window = WindowBuilder::new()
        .with_title("Raycast Renderer")
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32))
        .with_resizable(false)
        .build(&event_loop)
        .expect("Unable to create window.");

    let mut winit_input = WinitInputHelper::new();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).unwrap()
    };

    println!("init");

    event_loop.run(move |event, _, control_flow| {

        control_flow.set_poll();

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {

                time = Instant::now();

                //println!("time: {} - oldtime: {}", time.elapsed().as_secs_f64(), old_time.elapsed().as_secs_f64());

                let mut delta_time = time.elapsed().as_secs_f64() - old_time.elapsed().as_secs_f64();

                // clear frame so theres no ghosting (like what you see when you noclip through the map in half-life)
                //screen.iter_mut().for_each(|x| *x = 0);

                // todo input
                if winit_input.update(&event) {
                    input(&mut player, &winit_input, delta_time)
                }

                update(&mut screen, &mut player, time.elapsed().as_secs_f64() - old_time.elapsed().as_secs_f64());
                render(&screen, pixels.get_frame_mut());

                old_time = time;

            },
            Event::WindowEvent { 
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control_flow.set_exit();
            },
            _ => ()
        }

    })

}

// TODO render with pixels and winit
fn render(framebuffer: &PixelBuf, render_buffer: &mut [u8]) {

    for (i, (render, frame)) in render_buffer.chunks_exact_mut(4).zip(framebuffer.iter()).enumerate() {
        let x = (i % SCREEN_WIDTH) as i16;
        let y = (i / SCREEN_HEIGHT) as i16;

        let (r, g, b, a) = unpack_color(frame);

        //println!("rendering pixel of colour: ({}, {}, {}, {})", r, g, b, a);

        render.copy_from_slice(&[r, g, b, a]);

    }

}

fn update(screen: &mut PixelBuf, player: &mut Player, delta_time: f64) {

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
        let perp_wall_dist: f64;

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

    // fps counter. Displays in standard output because i don't want to setup text rendering
    println!("FPS: {}", 1.0 / (delta_time * -1.0));

}

fn input(player: &mut Player, input: &WinitInputHelper, delta_time: f64) {

    let move_speed: f64 = MOVE_SPEED * delta_time;
    let rot_speed: f64 = ROT_SPEED * delta_time;

    // forward
    if input.key_held(VirtualKeyCode::W) {
        if MAP[(player.pos.x + player.dir.x * move_speed).floor() as usize][player.pos.y.floor() as usize] == 0 {
            player.pos.x += player.dir.x * move_speed
        }
        if MAP[player.pos.x.floor() as usize][(player.pos.y + player.dir.y * move_speed).floor() as usize] == 0 {
            player.pos.y += player.dir.y * move_speed
        }
    }

    // backward
    if input.key_held(VirtualKeyCode::S) {
        if MAP[(player.pos.x - player.dir.x * move_speed).floor() as usize][player.pos.y.floor() as usize] == 0 {
            player.pos.x -= player.dir.x * move_speed
        }
        if MAP[player.pos.x.floor() as usize][(player.pos.y - player.dir.y * move_speed).floor() as usize] == 0 {
            player.pos.y -= player.dir.y * move_speed
        }
    }

    // turn left
    if input.key_held(VirtualKeyCode::A) {
        // both camera direction and camera plane must be rotated
        // TODO learn vector rotation
        let old_dir_x = player.dir.x;

        player.dir.x *= rot_speed.cos() - player.dir.y * rot_speed.sin();
        player.dir.y = old_dir_x * rot_speed.sin() + player.dir.y * rot_speed.cos();

        let old_plane_x = player.plane.x;
        
        player.plane.x *= rot_speed.cos() - player.plane.y * rot_speed.sin();
        player.plane.y = old_plane_x * rot_speed.sin() + player.plane.y * rot_speed.cos();
    }

    // turn right
    if input.key_held(VirtualKeyCode::D) {
        // both camera direction and camera plane must be rotated
        // TODO learn vector rotation
        let old_dir_x = player.dir.x;

        player.dir.x *= -rot_speed.cos() - player.dir.y * -rot_speed.sin();
        player.dir.y = old_dir_x * -rot_speed.sin() + player.dir.y * -rot_speed.cos();

        let old_plane_x = player.plane.x;
        
        player.plane.x *= -rot_speed.cos() - player.plane.y * -rot_speed.sin();
        player.plane.y = old_plane_x * -rot_speed.sin() + player.plane.y * -rot_speed.cos();
    }

}

fn draw_line(framebuffer: &mut PixelBuf, x: usize, y0: usize, y1: usize, colour: u32) {

    println!("draw line: {}", colour);

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