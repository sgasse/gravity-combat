use std::f64::consts::PI;

use glam::Vec2;
use log::{debug, info};
use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, Event, HtmlCanvasElement, KeyboardEvent};

use crate::{physics::calculate_missile_path, Planet, Ship, CANVAS_NAME};

pub(crate) struct Dimensions {
    pub width: f64,
    pub height: f64,
}

pub(crate) fn get_window_dimensions() -> Dimensions {
    // Fixed dimensions.
    Dimensions {
        width: 1000.,
        height: 700.,
    }
}

pub(crate) fn set_canvas_size(canvas_name: &str, width: u32, height: u32) {
    let canvas = get_canvas(canvas_name).expect("failed to get canvas");

    canvas.set_width(width);
    canvas.set_height(height);
    debug!("Set canvas size to {width}x{height}");
}

pub(crate) fn handle_input() {
    let window = window().unwrap();

    let dimensions = get_window_dimensions();

    let planets = [
        Planet {
            position: Vec2::new(150., 340.),
            radius: 50.,
        },
        Planet {
            position: Vec2::new(170., 500.),
            radius: 45.,
        },
    ];
    let mut ships = [
        Ship {
            position: Vec2::new(40., 400.),
            x_size: -30.,
            y_size: 2.,
            color: JsValue::from_str("rgb(255 0 0)"),
            angle: 0.,
        },
        Ship {
            position: Vec2::new(250., 400.),
            x_size: 30.,
            y_size: 2.,
            color: JsValue::from_str("rgb(0 0 255)"),
            angle: PI,
        },
    ];

    let canvas = get_canvas(CANVAS_NAME).unwrap();
    let ctx = get_2d_context(&canvas).unwrap();

    clear_canvas_ctx(&ctx, &dimensions);
    render_frame(&ctx, &planets, &ships);

    let callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        let event = event.clone().dyn_into::<KeyboardEvent>().unwrap();
        info!("Received key press: {:?}", event.key());

        const ANGLE_INCREMENT: f64 = 0.01 * PI;
        if event.key() == "w" {
            ships[0].angle += ANGLE_INCREMENT;
        } else if event.key() == "s" {
            ships[0].angle -= ANGLE_INCREMENT;
        } else if event.key() == "o" {
            ships[1].angle -= ANGLE_INCREMENT;
        } else if event.key() == "l" {
            ships[1].angle += ANGLE_INCREMENT;
        }

        ships[0].angle = ships[0].angle.clamp(-PI, PI);
        ships[1].angle = ships[1].angle.clamp(0., 2. * PI);

        clear_canvas_ctx(&ctx, &dimensions);
        render_frame(&ctx, &planets, &ships);

        if event.key() == "x" {
            let (path, result) = calculate_missile_path(
                ships[0].position,
                ships[0].angle as f32,
                ships[1].position,
                Vec2::new(dimensions.width as f32, dimensions.height as f32),
                &planets,
            );

            debug!("Missile result: {:?}", result);
            draw_missle_path(&ctx, &path);
        } else if event.key() == "." {
            let (path, result) = calculate_missile_path(
                ships[1].position,
                ships[1].angle as f32,
                ships[0].position,
                Vec2::new(dimensions.width as f32, dimensions.height as f32),
                &planets,
            );

            debug!("Missile result: {:?}", result);
            draw_missle_path(&ctx, &path);
        }
    });

    window
        .add_event_listener_with_callback("keypress", callback.as_ref().unchecked_ref())
        .unwrap();

    callback.forget();
}

fn get_canvas(canvas_name: &str) -> Option<HtmlCanvasElement> {
    window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(canvas_name))
        .and_then(|c| c.dyn_into::<HtmlCanvasElement>().ok())
}

fn get_2d_context(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
    canvas
        .get_context("2d")
        .ok()
        .flatten()
        .and_then(|x| x.dyn_into::<CanvasRenderingContext2d>().ok())
}

fn clear_canvas_ctx(ctx: &CanvasRenderingContext2d, dimensions: &Dimensions) {
    ctx.clear_rect(0., 0., dimensions.width, dimensions.height);
}

fn render_frame(ctx: &CanvasRenderingContext2d, planets: &[Planet], ships: &[Ship]) {
    clear_canvas_ctx(ctx, &get_window_dimensions());
    draw_planets(ctx, planets);
    draw_ships(ctx, ships);
}

fn draw_planets(ctx: &CanvasRenderingContext2d, planets: &[Planet]) {
    for planet in planets {
        ctx.set_fill_style(&JsValue::from_str("rgb(0 0 0)"));
        ctx.begin_path();
        ctx.ellipse(
            planet.position.x as f64,
            planet.position.y as f64,
            planet.radius,
            planet.radius,
            0.,
            0.,
            2. * PI,
        )
        .unwrap();
        ctx.fill();
    }
}

fn draw_ships(ctx: &CanvasRenderingContext2d, ships: &[Ship]) {
    for ship in ships {
        ctx.set_fill_style(&ship.color);
        ctx.begin_path();

        ctx.translate(ship.position.x as f64, ship.position.y as f64)
            .unwrap();

        ctx.rotate(ship.angle).unwrap();
        ctx.fill_rect(
            -ship.x_size / 2.,
            -ship.y_size / 2.,
            ship.x_size,
            ship.y_size,
        );
        ctx.fill_rect(-8., -8., 16., 16.);
        ctx.rotate(-ship.angle).unwrap();

        ctx.translate(-ship.position.x as f64, -ship.position.y as f64)
            .unwrap();
    }
}

fn draw_missle_path(ctx: &CanvasRenderingContext2d, missile: &[Vec2]) {
    ctx.set_fill_style(&JsValue::from_str("rgb(120 30 50)"));
    ctx.move_to(missile[0].x as f64, missile[0].y as f64);
    ctx.begin_path();

    for point in &missile[1..] {
        ctx.line_to(point.x as f64, point.y as f64);
    }

    ctx.stroke();
}
