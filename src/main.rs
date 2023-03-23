use std::collections::HashMap;
use std::time::Instant;

use pixels::wgpu::Color;
use winit::event_loop::EventLoop;
use winit::event::{Event, WindowEvent, VirtualKeyCode};
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;

use winit_input_helper::WinitInputHelper;

use pixels::{Error, Pixels, SurfaceTexture};

const MAP_HEIGHT: usize = 24;
const MAP_WIDTH: usize = 24;

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

const WALL_HEIGHT: i32 = 200;

const MOVE_SPEED: f64 = 500.0;
const ROT_SPEED: f64 = 30000.0;

#[derive(Debug, Clone, Copy)]
enum WallColours {
    Red = 0xff0000ff,
    Green = 0x00ff00ff,
    Blue = 0x0000ffff,
    White = 0xffffffff,
    Teal = 0x9ffcfdff,
    Yellow = 0xfffd85ff
}

type WallColour = [u8; 4];
type PixelBuf = Vec<WallColour>;

#[derive(Debug)]
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

#[derive(Debug)]
struct Player {
    pos: Vec2<f64>,
    dir: Vec2<f64>,
    plane: Vec2<f64>
}


fn main() {
    
    // this array is too big and causes a stack overflow
    let mut screen: PixelBuf = Vec::with_capacity(SCREEN_HEIGHT * SCREEN_WIDTH);
    unsafe { screen.set_len(SCREEN_HEIGHT * SCREEN_WIDTH) };

    // initialise player structure
    let mut player = Player {
        pos: Vec2::<f64>::new(12.0, 12.0),
        dir: Vec2::<f64>::new(-1.0, 0.0),
        plane: Vec2::<f64>::new(0.0, 0.66)
    };

    // time for fps counter
    let mut time: Instant = Instant::now();
    let mut old_time: Instant = Instant::now();

    // setup winit
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Raycast Renderer")
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32))
        .with_resizable(true)
        .build(&event_loop)
        .expect("Unable to create window.");

    let mut winit_input = WinitInputHelper::new();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {

        //control_flow.set_poll();
        if winit_input.update(&event) {

            let delta_time = (time.elapsed().as_secs_f64() - old_time.elapsed().as_secs_f64()).abs();

            input(&mut player, &winit_input, delta_time);

            if winit_input.key_pressed(VirtualKeyCode::Escape) {
                control_flow.set_exit();
            }

        }

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {

                time = Instant::now();

                //println!("time: {} - oldtime: {}", time.elapsed().as_secs_f64(), old_time.elapsed().as_secs_f64());

                let delta_time = (time.elapsed().as_secs_f64() - old_time.elapsed().as_secs_f64()).abs();

                // clear frame so theres no ghosting (like what you see when you noclip through the map in half-life)
                screen.iter_mut().for_each(|x| *x = [0xff, 0xff, 0xff, 0xff]);

                update(&mut screen, &mut player, delta_time);

                //screen.iter_mut().map(|x| *x = pack_colour(0xff, 0x8a, 0x8a));

                render(&screen, pixels.get_frame_mut());

                /*
                for pixel in pixels.get_frame_mut().chunks_exact_mut(4) {
                    pixel.copy_from_slice(&[0x5e, 0x48, 0xe8, 0xff]);
                }
                */

                pixels.render().expect("Render failed");

                old_time = time;

            },
            Event::WindowEvent { event: window_event, .. } => match window_event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                //WindowEvent::Resized(new_size) => pixels.resize_surface(new_size.width, new_size.height).unwrap(),
                _ => ()
            },
            _ => ()
        }

    })

}

fn render(framebuffer: &PixelBuf, render_buffer: &mut [u8]) {

    for (i, pixel) in render_buffer.chunks_exact_mut(4).enumerate() {
        let x = (i % SCREEN_WIDTH) as usize;
        let y = (i / SCREEN_HEIGHT) as usize;

        let index = x + y * SCREEN_WIDTH;

        if index >= framebuffer.len() {
            continue;
        }

        let pixel_colour = framebuffer[index];

        pixel.copy_from_slice(&pixel_colour);

    }

}

fn update(screen: &mut PixelBuf, player: &mut Player, delta_time: f64) {

    for x in 0..SCREEN_WIDTH {

        let camera_x: f64 = 2.0 * x as f64 / SCREEN_WIDTH as f64 - 1.0;
        let ray_dir = Vec2::<f64>::new(player.dir.x + player.plane.x * camera_x, player.dir.y + player.plane.y * camera_x);
        //let ray_dir_x: f64 = player.dir.x + player.plane.x * camera_x;

        // DDA setup
        let mut map_pos = Vec2::<i32>::new(player.pos.x.floor() as i32, player.pos.y.floor() as i32);
        let mut side_dist = Vec2::<f64>::new(0.0, 0.0);

        let delta_dist = Vec2::<f64>::new(if ray_dir.x == 0.0 { f64::INFINITY } else { (1.0 / ray_dir.x).abs() }, if ray_dir.y == 0.0 { f64::INFINITY } else { (1.0 / ray_dir.y).abs() });
        let mut perp_wall_dist: f64;

        let mut step = Vec2::<i32>::new(ray_dir.x.signum() as i32, ray_dir.y.signum() as i32);
        
        let mut hit: bool = false;
        let mut side: u8 = 0;

        // get initial step and side dist values
        if ray_dir.x < 0.0 {
            side_dist.x = (player.pos.x - map_pos.x as f64) * delta_dist.x;
        } else {
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

            assert!(map_pos.x as usize <= MAP_WIDTH && map_pos.y as usize <= MAP_HEIGHT, "raycast out of bounds.");

            if MAP[map_pos.x as usize][map_pos.y as usize] > 0 {
                hit = true;
            }

        }

        /* 
        if side == 0 {
            perp_wall_dist = side_dist.x - delta_dist.x;
        } else {
            perp_wall_dist = side_dist.y - delta_dist.y;
        }
        */

        /* perp_wall_dist = (x / 10) as f64;

        if perp_wall_dist as i32 == 0 { perp_wall_dist = 1.0 }

        */

        if side == 0 {
            perp_wall_dist = side_dist.x - delta_dist.x;
        } else {
            perp_wall_dist = side_dist.y - delta_dist.y;
        }

        //println!("perp wall dist: {perp_wall_dist}");

        let line_height: i32 = SCREEN_HEIGHT as i32 / perp_wall_dist as i32;

       // let line_height: i32 = WALL_HEIGHT / ((player.pos.x - perp_wall_dist).powi(2) + (player.pos.y - perp_wall_dist).powi(2)).sqrt() as i32;


        println!("perp wall dist: {perp_wall_dist}");

        let mut draw_start: i32 = -line_height / 2 + SCREEN_HEIGHT as i32 / 2;
        let mut draw_end: i32 = line_height / 2 + SCREEN_HEIGHT as i32 / 2;

        if draw_start < 0 {
            println!("draw start");
            draw_start = 0
        }

        if draw_end > SCREEN_HEIGHT as i32 {
            println!("exceeded wall height");
            draw_end = SCREEN_HEIGHT as i32 - 1;
        }

        let wall_colour_packed = match MAP[map_pos.x as usize][map_pos.y as usize] {
            1 => WallColours::Red as u32,
            2 => WallColours::Green as u32,
            3 => WallColours::Blue as u32,
            4 => WallColours::Teal as u32,
            _ => {
                println!("E");
                println!("hit: {}", MAP[map_pos.x as usize][map_pos.y as usize]);
                WallColours::White as u32
            }
        };

        let mut wall_colour = unpack_colour(&wall_colour_packed);

        // give walls a different brightness
        if side == 1 {
            for colour in wall_colour.iter_mut() {
                //*colour /= 2;
            }
        }

        //draw_line(screen, x, 200, 250, wall_colour)
        //draw_line(screen, x, SCREEN_HEIGHT / 2 - WALL_HEIGHT as usize / 2, SCREEN_HEIGHT / 2 + WALL_HEIGHT as usize / 2, wall_colour)
        draw_line(screen, x, draw_start as usize, draw_end as usize, wall_colour);

    }

    // fps counter. Displays in standard output because i don't want to setup text rendering
    //println!("FPS: {}", 1.0 / (delta_time * -1.0));

}

fn input(player: &mut Player, input: &WinitInputHelper, delta_time: f64) {

    let move_speed: f64 = MOVE_SPEED * delta_time;
    let rot_speed: f64 = ROT_SPEED * delta_time;

    // forward
    if input.key_held(VirtualKeyCode::W) {

        let wish_pos = Vec2::<f64>::new(player.pos.x + player.dir.x * move_speed, player.pos.y + player.dir.y * move_speed);

        if wish_pos.x.floor() as usize <= MAP_WIDTH && MAP[wish_pos.x.floor() as usize][player.pos.y.floor() as usize] == 0 {
            player.pos.x += player.dir.x * move_speed
        }


        if wish_pos.y.floor() as usize <= MAP_HEIGHT && MAP[player.pos.x.floor() as usize][wish_pos.y.floor() as usize] == 0 {
            player.pos.y += player.dir.y * move_speed
        }
    }

    // backward
    if input.key_held(VirtualKeyCode::S) {

        let wish_pos = Vec2::<f64>::new(player.pos.x - player.dir.x * move_speed, player.pos.y - player.dir.y * move_speed);

        if wish_pos.x.floor() as usize <= MAP_WIDTH && MAP[wish_pos.x.floor() as usize][player.pos.y.floor() as usize] == 0 {
            player.pos.x -= player.dir.x * move_speed
        }
        if wish_pos.y.floor() as usize <= MAP_HEIGHT && MAP[player.pos.x.floor() as usize][wish_pos.y.floor() as usize] == 0 {
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

fn draw_line(framebuffer: &mut PixelBuf, x: usize, y0: usize, y1: usize, colour: WallColour) {

    //println!("draw line: {}", colour);

    for y in y0..y1 {
        //println!("drawing at ({}, {}), buffer position {}",  x, y0, x + y * SCREEN_WIDTH);
        framebuffer[x + y * SCREEN_WIDTH] = colour;
    }

}

// TODO
fn pack_colour(r: u8, g: u8, b: u8) -> u32 {

    let packed = ((0 as u32) << 24) + ((b as u32) << 16) + ((g as u32) << 8) + (r as u32);

    return packed

}

fn unpack_colour(colour: &u32) -> [u8; 4] {

    let r = ((colour >>  0) & 255) as u8;
    let g = ((colour >>  8) & 255) as u8;
    let b = ((colour >> 16) & 255) as u8;
    let a = ((colour >> 24) & 255) as u8;

    [r, g, b, a]

}