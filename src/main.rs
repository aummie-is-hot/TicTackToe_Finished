/*
By: <aum markandey>
Date: 2026-10-02
Program Details: Tic tac toe game
*/

mod ui;
mod utils;
use core::num;
use std::result;

use crate::ui::image_button::ImageButton;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;
use crate::utils::preload_image::LoadingScreenOptions;
use crate::utils::preload_image::TextureManager;
use macroquad::input::KeyCode::B;
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
    async fn click(is_x_turn: &mut bool, btn_box: &mut ImageButton, num : &mut i32) -> i32 {
        if *is_x_turn {
            *is_x_turn = false;
            btn_box.set_image("assets/x.png", "assets/x.png").await;
            btn_box.enabled = false;
             *num = 1;
             return *num;
        } else {
            *is_x_turn = true;
            btn_box.set_image("assets/o.png", "assets/o.png").await;
            btn_box.enabled = false; // Disable the button after it has been clicked
            *num = 0;
             return *num;
        }
    }
    let mut btn_Exit = TextButton::new(1500.0, 1100.0, 200.0, 60.0, "Exit", WHITE, RED, 30);

    let mut btn_Reset = TextButton::new(600.0, 1100.0, 200.0, 60.0, "Reset", WHITE, RED, 30);
    

    texture_manager
        .preload_with_loading_screen(&["assets/o.png", "assets/x.png","assets/placeholder.png"], None, None)
        .await;
    btn_Exit.with_text_color(BLACK); // Sets the normal text color
    btn_Exit.with_hover_text_color(WHITE);
    btn_Reset.with_text_color(BLACK); // Sets the normal text color
    btn_Reset.with_hover_text_color(WHITE);
     let lbl_score = Label::new("score: 0-0", 850.0, 1000.0, 30);
    let mut btn_box1 = ImageButton::new(
        500.0,          // x position
        50.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/placeholder.png", // normal state image
        "assets/placeholder.png", // hover state image
    )
    .await;
let mut btn_box2 = ImageButton::new(
        800.0,          // x position
        50.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/placeholder.png", // normal state image
        "assets/placeholder.png", // hover state image
    )
    .await;
let mut btn_box3 = ImageButton::new(
        1100.0,          // x position
        50.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/placeholder.png", // normal state image
        "assets/placeholder.png", // hover state image
    )
    .await;
let mut btn_box4 = ImageButton::new(
        500.0,          // x position
        350.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/placeholder.png", // normal state image
        "assets/placeholder.png", // hover state image
    )
    .await;
let mut btn_box5 = ImageButton::new(
        800.0,          // x position
        350.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/placeholder.png", // normal state image
        "assets/placeholder.png", // hover state image
    )
    .await;
let mut btn_box6 = ImageButton::new(
        1100.0,          // x position
        350.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/placeholder.png", // normal state image
        "assets/placeholder.png", // hover state image
    )
    .await;
let mut btn_box7 = ImageButton::new(
        500.0,          // x position
        650.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/placeholder.png", // normal state image
        "assets/placeholder.png", // hover state image
    )
    .await;
let mut btn_box8 = ImageButton::new(
        800.0,          // x position
        650.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/placeholder.png", // normal state image
        "assets/placeholder.png", // hover state image
    )
    .await;
let mut btn_box9 = ImageButton::new(
        1100.0,         // x position
        650.0,          // y position
        200.0,          // width
        200.0,          // height
        "assets/placeholder.png", // normal state image
        "assets/placeholder.png", // hover state image
    )
    .await;
let mut is_x_turn = true;

let mut num = 0;
    loop {
        clear_background(WHITE);


        if btn_box1.click() {
            click(&mut is_x_turn, &mut btn_box1, &mut num).await;
            
      
        }
         

        if btn_box2.click() {
            click(&mut is_x_turn, &mut btn_box2, &mut num).await;
            num = num;
        }
        if btn_box3.click() {
           click(&mut is_x_turn, &mut btn_box3, &mut num).await;
        }
        if btn_box4.click() {
            click(&mut is_x_turn, &mut btn_box4, &mut num).await;
        }
        if btn_box5.click() {
            click(&mut is_x_turn, &mut btn_box5, &mut num).await;
        }
        if btn_box6.click() {
            click(&mut is_x_turn, &mut btn_box6, &mut num).await;
        }
        if btn_box7.click() {
            click(&mut is_x_turn, &mut btn_box7, &mut num).await;
        }
        if btn_box8.click() {
            click(&mut is_x_turn, &mut btn_box8, &mut num).await;
        }
        if btn_box9.click() {
            click(&mut is_x_turn, &mut btn_box9, &mut num).await;
        }
        if btn_Exit.click() {
            break;
        }
        if btn_Reset.click() {
            println!("{}", num);
        }
        lbl_score.draw();

        next_frame().await;
    }
}
