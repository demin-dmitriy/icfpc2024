use macroquad::prelude::*;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self}
};

const FONT_SIZE: f32 = 15.0;

fn fade(color: Color, a: f32) -> Color {
    Color::new(color.r, color.g, color.b, a)
}

fn draw_help() {
    let help_height = 90.0;
    let help_width = 380.0;
    draw_rectangle(
        10.0,
        10.0,
        help_width,
        help_height,
        fade(SKYBLUE, 0.5)
    );
    draw_rectangle_lines(10.0, 10.0, help_width, help_height, 2.0, BLUE);

    draw_text("Camera controls:", 20.0, 25.0, FONT_SIZE, BLACK);
    draw_text("- Right/Left/Up/Down or", 40.0, 45.0, FONT_SIZE, DARKGRAY);
    draw_text("  move mouse with Left Button pressed to move camera", 40.0, 65.0, FONT_SIZE, DARKGRAY);
    draw_text("- Z/X or use mouse wheel to Zoom in-out", 40.0, 85.0, FONT_SIZE, DARKGRAY);
}

fn draw_params_widget(
    data: &mut String,
    number: &mut f32,
    text: &mut String,
) {
    widgets::Window::new(hash!(), vec2(470., 50.), vec2(300., 300.))
        .label("Params")
        .ui(&mut *root_ui(), |ui| {

            ui.tree_node(hash!(), "input", |ui| {
                ui.label(None, "Some random text");
                if ui.button(None, "click me") {
                    println!("hi");
                }

                ui.separator();

                ui.input_text(hash!(), "<- input text", data);
                ui.label(
                    None,
                    &format!("Text entered: \"{}\"", data),
                );

                ui.separator();
            });
            ui.tree_node(hash!(), "sliders", |ui| {
                ui.slider(hash!(), "[-10 .. 10]", -10f32..10f32, number);
            });
            ui.tree_node(hash!(), "editbox", |ui| {
                ui.label(None, "This is editbox!");
                ui.editbox(hash!(), vec2(285., 165.), text);
            });
        });
}

fn draw_objects_on_scene() {
    draw_circle(
        0.0,
        0.0,
        0.1,
        BLUE
    );
}

fn move_camera(camera: &mut Camera2D, last_mouse_position: &mut Vec2) {

    let move_speed = 0.05;
    let wheel_speed = 1.1;

    let mut zoom = camera.zoom.x;

    let can_move_by_mouse = !root_ui().is_mouse_over(mouse_position().into());

    if can_move_by_mouse && is_mouse_button_pressed(MouseButton::Left) {
        *last_mouse_position = camera.screen_to_world(mouse_position().into());
    }

    if can_move_by_mouse && is_mouse_button_down(MouseButton::Left) {
        let mouse_position: Vec2 = camera.screen_to_world(mouse_position().into());
        let mouse_delta = mouse_position - *last_mouse_position;
        camera.offset.x += mouse_delta.x;
        camera.offset.y += mouse_delta.y;
    }

    if is_key_down(KeyCode::Left) {
        camera.offset.x += move_speed;
    }
    if is_key_down(KeyCode::Right) {
        camera.offset.x -= move_speed;
    }
    if is_key_down(KeyCode::Up) {
        camera.offset.y += move_speed;
    }
    if is_key_down(KeyCode::Down) {
        camera.offset.y -= move_speed;
    }

    if is_key_down(KeyCode::Z) {
        zoom *= wheel_speed;
    }
    if is_key_down(KeyCode::X) {
        zoom /= wheel_speed;
    }

    if can_move_by_mouse {
        match mouse_wheel() {
            (_x, y) if y != 0.0 => {
                zoom *= wheel_speed.powf(y);
            }
            _ => (),
        }
    }

    camera.zoom = vec2(zoom, zoom * screen_width() / screen_height());

    set_camera(camera);
}

#[macroquad::main("BasicShapes")]
async fn main() {

    let mut camera = Camera2D::default();
    let mut last_mouse_position: Vec2 = camera.screen_to_world(mouse_position().into());

    let mut data = String::new();
    let mut text = String::new();
    let mut number = 0.0;

    loop {
        clear_background(WHITE);

        move_camera(&mut camera, &mut last_mouse_position);

        draw_objects_on_scene();

        set_default_camera();

        draw_help();
        draw_params_widget(&mut data, &mut number, &mut text);

        next_frame().await
    }
}