use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

use super::constants::{WORLD_HEIGHT, WORLD_WIDTH, WORLD_X_UPPER_BOUND, WORLD_Y_UPPER_BOUND};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ParticleType {
    Empty,
    Sand,
    Water,
    Wood,
    Iron,
    Fire,
    Acid,
    Smoke,
    Steam,
    Lava,
}

#[derive(Copy, Clone)]
pub struct Particle {
    pub pt: ParticleType,
    pub should_burn: bool,
    pub updated: bool,
}

pub type Particles = Vec<Particle>;

pub fn point_to_index(x: u32, y: u32) -> usize {
    let h = WORLD_Y_UPPER_BOUND - y;
    let w = WORLD_X_UPPER_BOUND - x;

    return ((h * WORLD_WIDTH) + w) as usize;
}

pub fn index_to_point(index: usize) -> (u32, u32) {
    let w = index as u32 % WORLD_WIDTH;
    let rem = index as u32 - w;
    let h = rem / WORLD_WIDTH;

    return (WORLD_X_UPPER_BOUND - w, WORLD_Y_UPPER_BOUND - h);
}

fn check_if_type(x: u32, y: u32, particles: &Particles, pt: ParticleType) -> bool {
    if x >= WORLD_WIDTH || y >= WORLD_HEIGHT {
        return false;
    }

    let index = point_to_index(x, y);
    let particle = particles[index];

    return particle.pt == pt;
}

fn check_if_empty(x: u32, y: u32, particles: &Particles) -> bool {
    return check_if_type(x, y, particles, ParticleType::Empty);
}

pub fn update_particles(particles: &mut Particles) {
    for i in 1..particles.len() {
        particles[i].updated = false;
    }

    for i in 1..particles.len() {
        let mut particle = particles[i];
        let (x, y) = index_to_point(i);

        if particle.updated {
            continue;
        }

        particle.updated = true;

        match particle.pt {
            ParticleType::Sand => update_sand(x, y, particles),
            ParticleType::Water => update_water(x, y, particles),
            ParticleType::Empty => {}
            ParticleType::Wood => {}
            ParticleType::Iron => {}
            ParticleType::Fire => update_fire(x, y, particles),
            ParticleType::Lava => update_lava(x, y, particles),
            ParticleType::Acid => update_acid(x, y, particles),
            ParticleType::Smoke => update_smoke(x, y, particles),
            ParticleType::Steam => update_steam(x, y, particles),
        }
    }
}

pub fn draw_particles(particles: &Particles, canvas: &mut WindowCanvas) {
    for (i, particle) in particles.iter().enumerate() {
        let (x, y) = index_to_point(i);

        match particle.pt {
            ParticleType::Empty => {}
            ParticleType::Sand => {
                canvas.set_draw_color(Color::YELLOW);
                canvas.draw_point(Point::new(x as i32, y as i32));
            }
            ParticleType::Water => {
                canvas.set_draw_color(Color::BLUE);
                canvas.draw_point(Point::new(x as i32, y as i32));
            }
            ParticleType::Wood => {
                canvas.set_draw_color(Color::RGB(165, 42, 42));
                canvas.draw_point(Point::new(x as i32, y as i32));
            }
            ParticleType::Iron => {
                canvas.set_draw_color(Color::RGB(192, 192, 192));
                canvas.draw_point(Point::new(x as i32, y as i32));
            }
            ParticleType::Fire => {
                canvas.set_draw_color(Color::RED);
                canvas.draw_point(Point::new(x as i32, y as i32));
            }
            ParticleType::Acid => {
                canvas.set_draw_color(Color::GREEN);
                canvas.draw_point(Point::new(x as i32, y as i32));
            }
            ParticleType::Smoke => {
                canvas.set_draw_color(Color::GRAY);
                canvas.draw_point(Point::new(x as i32, y as i32));
            }
            ParticleType::Steam => {
                canvas.set_draw_color(Color::WHITE);
                canvas.draw_point(Point::new(x as i32, y as i32));
            }
            ParticleType::Lava => {
                canvas.set_draw_color(Color::MAGENTA);
                canvas.draw_point(Point::new(x as i32, y as i32));
            }
        }
    }
}

fn gen_adjacent_cell_list(x: u32, y: u32) -> Vec<(u32, u32)> {
    let x = x as i32;
    let y = y as i32;

    let cells = vec![
        (x + 1, y),
        (x - 1, y),
        (x, y + 1),
        (x, y - 1),
        (x + 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x - 1, y - 1),
    ];

    return cells
        .into_iter()
        .filter(|c| {
            c.0 >= 0
                && c.1 >= 0
                && c.0 <= (WORLD_WIDTH as i32 - 1)
                && c.1 <= (WORLD_HEIGHT as i32 - 1)
        })
        .map(|c| (c.0 as u32, c.1 as u32))
        .collect();
}

fn swap(i: usize, j: usize, particles: &mut Particles) {
    let temp = particles[i];
    particles[i] = particles[j];
    particles[j] = temp;
    particles[j].updated = true;
}

fn solid_particle_movement(x: u32, y: u32, particles: &mut Particles) {
    // Check if empty below
    if check_if_empty(x, y + 1, &particles) {
        let orig = point_to_index(x, y);
        let dest = point_to_index(x, y + 1);
        swap(orig, dest, particles);
        return;
    }

    // Check if water below
    if check_if_type(x, y + 1, &particles, ParticleType::Water) {
        let orig = point_to_index(x, y);
        let dest = point_to_index(x, y + 1);
        swap(orig, dest, particles);
        return;
    }

    if x > 0 && check_if_empty(x - 1, y + 1, &particles) {
        let orig = point_to_index(x, y);
        let dest = point_to_index(x - 1, y + 1);
        swap(orig, dest, particles);
        return;
    }

    if check_if_empty(x + 1, y + 1, &particles) {
        let orig = point_to_index(x, y);
        let dest = point_to_index(x + 1, y + 1);
        swap(orig, dest, particles);
        return;
    }
}

fn liquid_particle_movement(x: u32, y: u32, particles: &mut Particles) {
    if check_if_empty(x, y + 1, &particles) {
        let orig = point_to_index(x, y);
        let dest = point_to_index(x, y + 1);
        swap(orig, dest, particles);
        return;
    }

    if x > 0 && check_if_empty(x - 1, y + 1, &particles) {
        let orig = point_to_index(x, y);
        let dest = point_to_index(x - 1, y + 1);
        swap(orig, dest, particles);
        return;
    }

    if check_if_empty(x + 1, y + 1, &particles) {
        let orig = point_to_index(x, y);
        let dest = point_to_index(x + 1, y + 1);
        swap(orig, dest, particles);
        return;
    }

    if x > 0 && check_if_empty(x - 1, y, &particles) {
        let orig = point_to_index(x, y);
        let dest = point_to_index(x - 1, y);
        swap(orig, dest, particles);
        return;
    }

    if check_if_empty(x + 1, y, &particles) {
        let orig = point_to_index(x, y);
        let dest = point_to_index(x + 1, y);
        swap(orig, dest, particles);
        return;
    }
}

fn gas_particle_movement(x: u32, y: u32, particles: &mut Particles) {
    if y == 0 {
        return;
    }
    if check_if_empty(x, y - 1, &particles) {
        let orig = point_to_index(x, y);
        let dest = point_to_index(x, y - 1);
        swap(orig, dest, particles);
        return;
    }

    if x > 0 && check_if_empty(x - 1, y - 1, &particles) {
        let orig = point_to_index(x, y);
        let dest = point_to_index(x - 1, y - 1);
        swap(orig, dest, particles);
        return;
    }

    if check_if_empty(x + 1, y - 1, &particles) {
        let orig = point_to_index(x, y);
        let dest = point_to_index(x + 1, y - 1);
        swap(orig, dest, particles);
        return;
    }

    if x > 0 && check_if_empty(x - 1, y, &particles) {
        let orig = point_to_index(x, y);
        let dest = point_to_index(x - 1, y);
        swap(orig, dest, particles);
        return;
    }

    if check_if_empty(x + 1, y, &particles) {
        let orig = point_to_index(x, y);
        let dest = point_to_index(x + 1, y);
        swap(orig, dest, particles);
        return;
    }
}

fn update_sand(x: u32, y: u32, particles: &mut Particles) {
    solid_particle_movement(x, y, particles);
}

fn update_water(x: u32, y: u32, particles: &mut Particles) {
    liquid_particle_movement(x, y, particles);
}

fn update_fire(x: u32, y: u32, particles: &mut Particles) {
    let index = point_to_index(x, y);
    let mut particle = particles[index];

    if !particle.should_burn {
        particle.should_burn = true;
        particles[index] = particle;
        return;
    }

    let cells = gen_adjacent_cell_list(x, y);

    let mut burnt = false;

    for (i, j) in cells {
        if check_if_type(i, j, particles, ParticleType::Wood) {
            let dest = point_to_index(i, j);
            particles[dest].pt = ParticleType::Fire;
            particles[dest].should_burn = false;
            burnt = true;
        }
    }

    if !burnt {
        particle.pt = ParticleType::Smoke;
        particle.should_burn = false;
    }

    particles[index] = particle;
}

fn update_lava(x: u32, y: u32, particles: &mut Particles) {
    let cells = gen_adjacent_cell_list(x, y);

    let mut evaporated = false;

    for (i, j) in cells {
        if check_if_type(i, j, particles, ParticleType::Wood) {
            let dest = point_to_index(i, j);
            particles[dest].pt = ParticleType::Fire;
            particles[dest].should_burn = false;
        }

        if check_if_type(i, j, particles, ParticleType::Water) {
            let index = point_to_index(x, y);
            particles[index].pt = ParticleType::Steam;

            let dest = point_to_index(i, j);
            particles[dest].pt = ParticleType::Empty;

            evaporated = true;
        }
    }

    if evaporated {
        return;
    }

    liquid_particle_movement(x, y, particles);
}

fn update_acid(x: u32, y: u32, particles: &mut Particles) {
    let index = point_to_index(x, y);
    let mut particle = particles[index];

    if !particle.should_burn {
        particle.should_burn = true;
        particles[index] = particle;
        return;
    }

    let cells = gen_adjacent_cell_list(x, y);

    for (i, j) in cells {
        if check_if_type(i, j, particles, ParticleType::Iron) {
            let dest = point_to_index(i, j);
            particles[dest].pt = ParticleType::Acid;
            particles[dest].should_burn = false;
            break;
        }
    }

    liquid_particle_movement(x, y, particles);
}

fn update_smoke(x: u32, y: u32, particles: &mut Particles) {
    gas_particle_movement(x, y, particles);
}

fn update_steam(x: u32, y: u32, particles: &mut Particles) {
    gas_particle_movement(x, y, particles);
}
