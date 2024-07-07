use std::f32::consts::PI;

use glam::Vec2;

use crate::{MissileResult, Planet};

pub(crate) fn calculate_missile_path(
    start_pos: Vec2,
    angle: f32,
    enemy_pos: Vec2,
    dimensions: Vec2,
    planets: &[Planet],
) -> (Vec<Vec2>, MissileResult) {
    // The missiles are shot at a constant speed at the current angle of the ship.
    // Planets apply force on the missiles through gravity.
    //
    // F_G = G * ((m_1 * m_2) / r^2)
    //
    // The forces can be turned into acceleration vectors with
    //
    // F = m * a <=> a = F / m
    //
    // => a = F_G / m_1 = (G * m_1 * m_2) / (r^2 * m_1) = (G * m_2) / r^2
    const V_INITIAL: f32 = 4.0;
    const G: f32 = 0.01;

    let mut position = start_pos;
    let mut path = vec![position];

    let mut velocity = Vec2::from_angle(angle).rotate(Vec2::X) * V_INITIAL;

    const TIME_INCREMENT: f32 = 0.5;

    loop {
        // Move one increment.
        position += velocity * TIME_INCREMENT;
        path.push(position);

        // Update velocity vector.
        for planet in planets {
            let m_planet = (4. / 3.) * PI * (planet.radius as f32).powi(3);
            let magnitude = (G * m_planet) / planet.position.distance_squared(position);

            let acceleration = (planet.position - position).normalize() * magnitude;

            velocity += acceleration * TIME_INCREMENT;
        }

        if let Some(stop_reason) =
            check_stop_reasons(position, enemy_pos, planets, dimensions, path.len())
        {
            return (path, stop_reason);
        }
    }
}

fn check_stop_reasons(
    position: Vec2,
    enemy_pos: Vec2,
    planets: &[Planet],
    dimensions: Vec2,
    path_length: usize,
) -> Option<MissileResult> {
    const MAX_PATH_LENGTH: usize = 300;
    const ENEMY_COLLISION_DISTANCE: f32 = 16.0;

    // Check for maximum travel time.
    if path_length >= MAX_PATH_LENGTH {
        return Some(MissileResult::OutofFuel);
    }

    // Check for leaving the screen.
    if position.x >= dimensions.x
        || position.y >= dimensions.y
        || position.x <= 0.
        || position.y <= 0.
    {
        return Some(MissileResult::OutOfRange);
    }

    // Check for collision with a star.
    for planet in planets {
        if position.distance(planet.position) <= planet.radius as f32 {
            return Some(MissileResult::HitPlanet);
        }
    }

    // Check for collision with enemy. The enemy is approximated as circle.
    if position.distance(enemy_pos) <= ENEMY_COLLISION_DISTANCE {
        return Some(MissileResult::HitEnemy);
    }

    None
}
