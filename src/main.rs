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
        window_width: 1050,
        window_height: 768,
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
   
    let mut btn_Exit = TextButton::new(550.0, 700.0, 200.0, 60.0, "Exit", WHITE, RED, 30);

    let mut btn_Reset = TextButton::new(250.0, 700.0, 200.0, 60.0, "Reset", WHITE, RED, 30);
    let mut btns = vec![ImageButton::new(
        300.0,  // x position
        100.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
    ImageButton::new(
        450.0,  // x position
        100.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
    ImageButton::new(
        600.0,  // x position
        100.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
    ImageButton::new(
        300.0,  // x position
        250.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
     ImageButton::new(
        450.0,  // x position
        250.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
     ImageButton::new(
        600.0,  // x position
        250.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,ImageButton::new(
        300.0,  // x position
        400.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
     ImageButton::new(
        450.0,  // x position
        400.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
     ImageButton::new(
        600.0,  // x position
        400.0,  // y position
        100.0,  // width
        100.0,   // height
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
     let mut lbl_score = Label::new("score: 0-0", 425.0, 600.0, 30);
      let mut lbl_Player = Label::new("Player: X", 100.0, 600.0, 30);
   let mut player = false;
    loop {
        clear_background(WHITE);
        
        for i in (0..btns.len()).step_by(3){
            if btns[i].get_filename() == "assets/x.png" && btns[i].get_filename() == btns[i+1].get_filename() && btns[i].get_filename() == btns[i+2].get_filename() && btns[i+1].get_filename() == btns[i+2].get_filename() && btns[i].get_filename() == btns[i+2].get_filename(){
                lbl_score.set_text(&format!("score"));
                println!("won")
            } 
        }
       
       
        for i in 0..btns.len(){
            if btns[i].click(){
                btns[i].enabled = false;
                if player == false {
                    btns[i].set_image("assets/x.png", "assets/x.png").await;
                    
                    player = true;
                    println!("{}",btns[i].get_filename());
                    lbl_Player.set_text("Player: O");
                } else {
                    btns[i].set_image("assets/o.png", "assets/o.png").await;
                    player = false;
                    
                    lbl_Player.set_text("Player: X");
                }
            }
        }
        lbl_score.draw();
        if btn_Exit.click(){
            break;
        }
         if btn_Reset.click(){
            for i in 0..btns.len(){
                btns[i].set_image("assets/placeholder.png", "assets/placeholder.png").await;
                player = false;
                btns[i].enabled = true;
                lbl_Player.set_text("Player: X");
            }
        }
        lbl_Player.draw();

        next_frame().await;
    }
}
