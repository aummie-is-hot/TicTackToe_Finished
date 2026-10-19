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
use macroquad::ui::DragState::Clicked;

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
    let mut btns = vec![ImageButton::new(
        100.0,  // x position
        200.0,  // y position
        200.0,  // width
        200.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
    ImageButton::new(
        400.0,  // x position
        200.0,  // y position
        200.0,  // width
        200.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
    ImageButton::new(
        400.0,  // x position
        200.0,  // y position
        200.0,  // width
        200.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
    ImageButton::new(
        400.0,  // x position
        200.0,  // y position
        200.0,  // width
        200.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await];

    texture_manager
        .preload_with_loading_screen(&["assets/o.png", "assets/x.png","assets/placeholder.png"], None, None)
        .await;
    btn_Exit.with_text_color(BLACK); // Sets the normal text color
    btn_Exit.with_hover_text_color(WHITE);
    btn_Reset.with_text_color(BLACK); // Sets the normal text color
    btn_Reset.with_hover_text_color(WHITE);
     let lbl_score = Label::new("score: 0-0", 850.0, 1000.0, 30);
   
    loop {
        clear_background(WHITE);
        for i in 0..btns.len(){
            if btns[i].click(){

            }
        }
        if btn_Exit.click(){
            break;
        }
         if btn_Reset.click(){
            break;
        }
        

        next_frame().await;
    }
}
