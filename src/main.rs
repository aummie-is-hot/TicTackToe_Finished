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
use macroquad::input::KeyCode::Y;
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
   let mut btn_Easy = TextButton::new(650.0, 400.0, 200.0, 60.0, "EASY", WHITE, GREEN, 30);
     let mut btn_Medium = TextButton::new(850.0, 400.0, 200.0, 60.0, "MEDIUM", WHITE, YELLOW, 30);
     let mut btn_Hard = TextButton::new(1050.0, 400.0, 200.0, 60.0, "HARD", WHITE, RED, 30);
    

    let mut btn_Human = TextButton::new(650.0, 200.0, 200.0, 60.0, "Human", WHITE, BLUE, 30);
     let mut btn_AI = TextButton::new(850.0, 200.0, 200.0, 60.0, "AI", WHITE, RED, 30);
    
    let mut btn_Exit = TextButton::new(400.0, 650.0, 200.0, 60.0, "Exit", WHITE, RED, 30);
    
 let mut lbl_Title = Label::new("TIC TAC TOE", 200.0, 100.0, 40);
    let mut btn_Reset = TextButton::new(150.0, 650.0, 200.0, 60.0, "Reset", WHITE, RED, 30);
    let mut btns = vec![ImageButton::new(
        200.0,  // x position
        190.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
    ImageButton::new(
        310.0,  // x position
        190.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
    ImageButton::new(
        420.0,  // x position
        190.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
    ImageButton::new(
        200.0,  // x position
        300.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
     ImageButton::new(
        310.0,  // x position
        300.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
     ImageButton::new(
        420.0,  // x position
        300.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,ImageButton::new(
       200.0,  // x position
        410.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
     ImageButton::new(
       310.0,  // x position
        410.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await,
     ImageButton::new(
        420.0,  // x position
       410.0,  // y position
        100.0,  // width
        100.0,   // height
        "assets/placeholder.png",        // normal state image
        "assets/placeholder.png",  // hover state image
    ).await];
     
   
    btn_Exit.with_text_color(BLACK); // Sets the normal text color
    btn_Exit.with_hover_text_color(WHITE);
    btn_Reset.with_text_color(BLACK); // Sets the normal text color
    btn_Reset.with_hover_text_color(WHITE);
     btn_Easy.with_text_color(BLACK); // Sets the normal text color
    btn_Easy.with_hover_text_color(WHITE);
    btn_Medium.with_text_color(BLACK); // Sets the normal text color
    btn_Medium.with_hover_text_color(BLACK);
     btn_Hard.with_text_color(BLACK); // Sets the normal text color
    btn_Hard.with_hover_text_color(WHITE);
      btn_AI.with_text_color(BLACK); // Sets the normal text color
    btn_AI.with_hover_text_color(WHITE);
    btn_Human.with_text_color(BLACK); // Sets the normal text color
    btn_Human.with_hover_text_color(WHITE);
     btn_Easy.with_border(GREEN, 5.0);
      btn_Medium.with_border(YELLOW, 5.0);
       btn_Hard.with_border(RED, 5.0);
        btn_Exit.with_border(RED, 5.0);
         btn_Reset.with_border(RED, 5.0);
          btn_AI.with_border(RED, 5.0);
           btn_Human.with_border(BLUE, 5.0);
     let mut lbl_score = Label::new("Score: X: 0 - 0", 650.0, 100.0, 30);
let mut lbl_Player = Label::new("Player: X", 650.0, 150.0, 30);
   let mut player = false;
   let mut x_score = 0;
    let mut o_score = 0;
    let mut game_over = false;
    loop {
        clear_background(WHITE);
        if game_over == true{
              for i in 0..btns.len(){
            btns[i].enabled = false;
         } 
        }
        lbl_score.set_text(&format!("Score: X: {} - {} :O", x_score,o_score));
        for i in (0..btns.len()).step_by(3){
            if btns[i].get_filename() != "assets/placeholder.png" && btns[i].get_filename() == btns[i+1].get_filename() && btns[i].get_filename() == btns[i+2].get_filename() && btns[i+1].get_filename() == btns[i+2].get_filename() && btns[i].get_filename() == btns[i+2].get_filename() {
                
                if game_over == false {
                    if btns[i].get_filename() == "assets/x.png" {
                
        x_score += 1;
        game_over = true;
         lbl_Player.set_text("X won!");
       
    } else {
        o_score += 1;
        game_over = true;
        lbl_Player.set_text("O won!");
    }
                }
                 
    btns[i].enabled = false;

            } 
             
        }
       for i in (0..3).step_by(1) { // dont do 0..btns len cuz then the index cant be greater than 2 before the program crashes as 3 plus 6 is nine and the length of btns is 8 so it crashes
    if btns[i].get_filename() != "assets/placeholder.png"// ||btns[i].get_filename() == "assets/o.png"
        && btns[i].get_filename() == btns[i+3].get_filename()
        && btns[i].get_filename() == btns[i+6].get_filename()
    {
       
         if game_over == false {
                    if btns[i].get_filename() == "assets/x.png" {
                
        x_score += 1;
        game_over = true;
         lbl_Player.set_text("X won!");
    } else {
        o_score += 1;
        game_over = true;
        lbl_Player.set_text("O won!");
    }
                }
    btns[i].enabled = false;
    
    }
}
for i in (2..3).step_by(2) {
    if btns[i].get_filename() != "assets/placeholder.png"
        && btns[i].get_filename() == btns[i+2].get_filename()
        && btns[i].get_filename() == btns[i+4].get_filename()
    {
       if game_over == false {
                    if btns[i].get_filename() == "assets/x.png" {
                
        x_score += 1;
        game_over = true;
        lbl_Player.set_text("X won!");
    } else {
        o_score += 1;
        game_over = true;
        lbl_Player.set_text("O won!");
    }
                }

        btns[i].enabled = false;
        
    }
}

for i in (2..3).step_by(2) {
   if btns[0].get_filename() != "assets/placeholder.png"
    && btns[0].get_filename() == btns[4].get_filename()
    && btns[0].get_filename() == btns[8].get_filename()
{
   if game_over == false {
                    if btns[0].get_filename() == "assets/x.png" {
                
        x_score += 1;
        game_over = true;
         lbl_Player.set_text("X won!");
    } else {
        o_score += 1;
        game_over = true;
        lbl_Player.set_text("O won!");
    }
                }

    btns[0].enabled = false;
   
}
}// add reset confirmation to reset score aswell or just gameboard

       
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
                game_over = false;
            }
        }
        lbl_Player.draw();
        lbl_Title.draw();
        if btn_AI.click(){

        } 
        if btn_Human.click(){
            
        }
         if btn_Easy.click(){
            
        }
         if btn_Medium.click(){
            
        }
         if btn_Hard.click(){
            
        }
        next_frame().await;
    }
}
// make animated like just hovering xs and os backgrounmd for the game and a main menu or nah