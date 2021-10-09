extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

use std::time::{Duration, Instant};

mod constants;
mod particles;

use constants::{MAX_INDEX, PIXEL_SCALE, WINDOW_HEIGHT, WINDOW_WIDTH, WORLD_HEIGHT, WORLD_WIDTH};
use particles::{
    draw_particles, point_to_index, update_particles, Particle, ParticleType, Particles,
};

const TARGET_TIME_PER_UPDATE: Duration = Duration::from_nanos(16666670);

struct InputState {
    draw_type: ParticleType,
}

fn setup() -> (WindowCanvas, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_scale(PIXEL_SCALE as f32, PIXEL_SCALE as f32);
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump().unwrap();

    (canvas, event_pump)
}

fn handle_input(event_pump: &EventPump, state: &mut InputState, particles: &mut Particles) {
    // Add new particles
    let key_state = event_pump.keyboard_state();

    if key_state.is_scancode_pressed(Scancode::E) {
        state.draw_type = ParticleType::Water;
    } else if key_state.is_scancode_pressed(Scancode::Q) {
        state.draw_type = ParticleType::Sand;
    } else if key_state.is_scancode_pressed(Scancode::R) {
        state.draw_type = ParticleType::Wood;
    } else if key_state.is_scancode_pressed(Scancode::T) {
        state.draw_type = ParticleType::Iron;
    } else if key_state.is_scancode_pressed(Scancode::F) {
        state.draw_type = ParticleType::Fire;
    } else if key_state.is_scancode_pressed(Scancode::A) {
        state.draw_type = ParticleType::Lava;
    } else if key_state.is_scancode_pressed(Scancode::W) {
        state.draw_type = ParticleType::Acid;
    } else if key_state.is_scancode_pressed(Scancode::D) {
        state.draw_type = ParticleType::Steam;
    } else if key_state.is_scancode_pressed(Scancode::G) {
        state.draw_type = ParticleType::Smoke;
    }

    let mouse_state = event_pump.mouse_state();

    if mouse_state.left() {
        let x = mouse_state.x();
        let y = mouse_state.y();

        let w = (x as u32 / PIXEL_SCALE) as i32;
        let h = (y as u32 / PIXEL_SCALE) as i32;

        for i in (w - 2)..(w + 2) {
            for j in (h - 2)..(h + 2) {
                if i >= 0 && i <= WORLD_WIDTH as i32 && j >= 0 && j <= WORLD_HEIGHT as i32 {
                    let index = point_to_index(i as u32, j as u32);
                    particles[index].pt = state.draw_type;
                }
            }
        }
    }
}

fn update(particles: &mut Particles) {
    // Find particles to update
    update_particles(particles);
}

fn draw(canvas: &mut WindowCanvas, particles: &Particles) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    // Draw particles
    draw_particles(&particles, canvas);

    canvas.present();
}

pub fn main() {
    let (mut canvas, mut event_pump) = setup();

    let empty_particle = Particle {
        pt: ParticleType::Empty,
        should_burn: false,
        updated: false,
    };

    let mut particles: Particles = vec![empty_particle; MAX_INDEX as usize];

    // Default to sand
    let mut input_state = InputState {
        draw_type: ParticleType::Sand,
    };

    let mut frame_start = Instant::now();

    // TODO: Better game loop timing
    'running: loop {
        frame_start = Instant::now();

        update(&mut particles);

        handle_input(&event_pump, &mut input_state, &mut particles);

        draw(&mut canvas, &particles);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }

    let diff = TARGET_TIME_PER_UPDATE - frame_start.elapsed();

    if diff.as_nanos() >= 0 {
        ::std::thread::sleep(diff);
    }
}
