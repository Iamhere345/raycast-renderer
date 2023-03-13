use winit::{event_loop::{EventLoop}, event::{Event, WindowEvent}, window::WindowBuilder, dpi::LogicalSize};
use winit::event_loop::EventLoop;
use winit::event::{Event, WindowEvent};
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;

use winit_input_helper::WinitInputHelper;

const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 480;

const MAP: [[u32; 0]; 0] =
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
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
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

    for x in 0..SCREEN_WIDTH {

        let camera_x: f64 = 2.0 * x as f64 / SCREEN_WIDTH as f64;
        let ray = Vec2::<f64>::new(player.dir.x + player.plane.x * camera_x, player.dir.y + player.plane.y * camera_x);
        let ray_dir_x: f64 = player.dir.x + player.plane.x * camera_x;

    }

}