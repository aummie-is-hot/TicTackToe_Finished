/*
By: <aum markandey>
Date: 2026-10-02
Program Details: Tic tac toe game
*/

mod ui;
mod utils;
use std::result;

use crate::ui::image_button::ImageButton;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;
use crate::utils::preload_image::LoadingScreenOptions;
use crate::utils::preload_image::TextureManager;
use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "tic tac toe".to_string(),
        window_width: 1800,
        window_height: 1200,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture_manager = TextureManager::new();

    let mut btn_Exit = TextButton::new(1500.0, 1100.0, 200.0, 60.0, "Exit", WHITE, RED, 30);

    let mut btn_Reset = TextButton::new(600.0, 1100.0, 200.0, 60.0, "Reset", WHITE, RED, 30);
    texture_manager
        .preload_with_loading_screen(&["assets/o.png", "assets/x.png"], None, None)
        .await;
    btn_Exit.with_text_color(BLACK); // Sets the normal text color
    btn_Exit.with_hover_text_color(WHITE);
    btn_Reset.with_text_color(BLACK); // Sets the normal text color
    btn_Reset.with_hover_text_color(WHITE);
    let btn_box1 = ImageButton::new(
        500.0,          // x position
        0.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/x.png", // normal state image
        "assets/x.png", // hover state image
    )
    .await;
let btn_box2 = ImageButton::new(
        800.0,          // x position
        0.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/x.png", // normal state image
        "assets/x.png", // hover state image
    )
    .await;
let btn_box3 = ImageButton::new(
        1100.0,          // x position
        0.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/x.png", // normal state image
        "assets/x.png", // hover state image
    )
    .await;
let btn_box4 = ImageButton::new(
        500.0,          // x position
        400.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/x.png", // normal state image
        "assets/x.png", // hover state image
    )
    .await;
let btn_box5 = ImageButton::new(
        800.0,          // x position
        400.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/x.png", // normal state image
        "assets/x.png", // hover state image
    )
    .await;
let btn_box6 = ImageButton::new(
        1100.0,          // x position
        400.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/x.png", // normal state image
        "assets/x.png", // hover state image
    )
    .await;
let btn_box7 = ImageButton::new(
        500.0,          // x position
        700.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/x.png", // normal state image
        "assets/x.png", // hover state image
    )
    .await;
let btn_box8 = ImageButton::new(
        800.0,          // x position
        700.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/x.png", // normal state image
        "assets/x.png", // hover state image
    )
    .await;
let btn_box9 = ImageButton::new(
        1100.0,         // x position
        700.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/x.png", // normal state image
        "assets/x.png", // hover state image
    )
    .await;

    loop {
        clear_background(WHITE);
        if btn_box1.click() {
            // Handle button click
        }
        if btn_box2.click() {
            // Handle button click
        }
        if btn_box3.click() {
            // Handle button click
        }
        if btn_box4.click() {
            // Handle button click
        }
        if btn_box5.click() {
            // Handle button click
        }
        if btn_box6.click() {
            // Handle button click
        }
        if btn_box7.click() {
            // Handle button click
        }
        if btn_box8.click() {
            // Handle button click
        }
        if btn_box9.click() {
            // Handle button click
        }
        if btn_Exit.click() {
            break;
        }
        if btn_Reset.click() {
            // Handle reset button click
        }
        next_frame().await;
    }
}
