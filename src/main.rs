/*
By: <aum markandey>
Date: 2026-10-02
Program Details: Tic tac toe game
*/

mod ui;
mod utils;

use crate::ui::image_button::ImageButton;
use crate::ui::label::Label;
use crate::ui::messagebox::{MessageBox, MessageBoxResult};
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;
use crate::utils::preload_image::LoadingScreenOptions;
use crate::utils::preload_image::TextureManager;
use macroquad::input::KeyCode::B;
use macroquad::input::KeyCode::Y;
use macroquad::prelude::*;
use macroquad::ui::DragState::Clicked;
use miniquad::date;
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "tic tac toe".to_string(),
        window_width: 1250,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}
fn ai_move(ai_diff: i32, vector: &mut Vec<ImageButton>, player: bool, tm: &TextureManager, movecount: i32) {
    if ai_diff == 1 {
        loop {
            let AImove = rand::gen_range(0, vector.len());
            if vector[AImove].get_filename() == "assets/placeholder.png" {
                if player == true {
                    vector[AImove].set_preload(tm.get_preload("assets/o.png").unwrap(), tm.get_preload("assets/o.png").unwrap());
                } else {
                    vector[AImove].set_preload(tm.get_preload("assets/o.png").unwrap(), tm.get_preload("assets/o.png").unwrap());
                }
                vector[AImove].enabled = false;
                break;
            } else if vector[0].get_filename() != "assets/placeholder.png"
            && vector[1].get_filename() != "assets/placeholder.png"
            && vector[2].get_filename() != "assets/placeholder.png"
            && vector[3].get_filename() != "assets/placeholder.png"
            && vector[4].get_filename() != "assets/placeholder.png"
            && vector[5].get_filename() != "assets/placeholder.png"
            && vector[6].get_filename() != "assets/placeholder.png"
            && vector[7].get_filename() != "assets/placeholder.png"
            && vector[8].get_filename() != "assets/placeholder.png"{
break;
            }
        }
        
    }

    if ai_diff == 2{
   loop {
            if movecount == 1{
                let AImove = rand::gen_range(0, vector.len());
            if vector[AImove].get_filename() == "assets/placeholder.png" {
                if player == true {
                    vector[AImove].set_preload(tm.get_preload("assets/o.png").unwrap(), tm.get_preload("assets/o.png").unwrap());
                } else {
                    vector[AImove].set_preload(tm.get_preload("assets/o.png").unwrap(), tm.get_preload("assets/o.png").unwrap());
                }
                vector[AImove].enabled = false;
                break;
            } else if vector[0].get_filename() != "assets/placeholder.png"
            && vector[1].get_filename() != "assets/placeholder.png"
            && vector[2].get_filename() != "assets/placeholder.png"
            && vector[3].get_filename() != "assets/placeholder.png"
            && vector[4].get_filename() != "assets/placeholder.png"
            && vector[5].get_filename() != "assets/placeholder.png"
            && vector[6].get_filename() != "assets/placeholder.png"
            && vector[7].get_filename() != "assets/placeholder.png"
            && vector[8].get_filename() != "assets/placeholder.png"{
break;
            }
        }

             if movecount == 2{
               



             }

        
            }
     
}

}
#[macroquad::main(window_conf)]
async fn main() {
    let mut movecount = 1;
    let tm = TextureManager::new();
    tm.preload_all(&["assets/x.png", "assets/o.png", "assets/placeholder.png"]).await;
    let mut btn_Easy = TextButton::new(650.0, 400.0, 200.0, 60.0, "EASY", WHITE, GREEN, 30);
    let mut btn_Medium = TextButton::new(850.0, 400.0, 200.0, 60.0, "MEDIUM", WHITE, YELLOW, 30);
    let mut btn_Hard = TextButton::new(1050.0, 400.0, 200.0, 60.0, "HARD", WHITE, RED, 30);

    let mut btn_Human = TextButton::new(650.0, 200.0, 200.0, 60.0, "Human", WHITE, BLUE, 30);
    let mut btn_AI = TextButton::new(850.0, 200.0, 200.0, 60.0, "AI", WHITE, RED, 30);

    let mut btn_Exit = TextButton::new(400.0, 650.0, 200.0, 60.0, "Exit", WHITE, RED, 30);

    let mut lbl_Title = Label::new("TIC TAC TOE", 200.0, 100.0, 40);
    let mut btn_Reset = TextButton::new(150.0, 650.0, 200.0, 60.0, "Reset", WHITE, RED, 30);
    let mut btns = vec![
        ImageButton::from_preload(
            200.0,                                             // x position
            190.0,                                             // y position
            100.0,                                             // width
            100.0,                                             // height
            tm.get_preload("assets/placeholder.png").unwrap(), // normal state image
            tm.get_preload("assets/placeholder.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            310.0,                                             // x position
            190.0,                                             // y position
            100.0,                                             // width
            100.0,                                             // height
            tm.get_preload("assets/placeholder.png").unwrap(), // normal state image
            tm.get_preload("assets/placeholder.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            420.0,                                             // x position
            190.0,                                             // y position
            100.0,                                             // width
            100.0,                                             // height
            tm.get_preload("assets/placeholder.png").unwrap(), // normal state image
            tm.get_preload("assets/placeholder.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            200.0,                                             // x position
            300.0,                                             // y position
            100.0,                                             // width
            100.0,                                             // height
            tm.get_preload("assets/placeholder.png").unwrap(), // normal state image
            tm.get_preload("assets/placeholder.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            310.0,                                             // x position
            300.0,                                             // y position
            100.0,                                             // width
            100.0,                                             // height
            tm.get_preload("assets/placeholder.png").unwrap(), // normal state image
            tm.get_preload("assets/placeholder.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            420.0,                                             // x position
            300.0,                                             // y position
            100.0,                                             // width
            100.0,                                             // height
            tm.get_preload("assets/placeholder.png").unwrap(), // normal state image
            tm.get_preload("assets/placeholder.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            200.0,                                             // x position
            410.0,                                             // y position
            100.0,                                             // width
            100.0,                                             // height
            tm.get_preload("assets/placeholder.png").unwrap(), // normal state image
            tm.get_preload("assets/placeholder.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            310.0,                                             // x position
            410.0,                                             // y position
            100.0,                                             // width
            100.0,                                             // height
            tm.get_preload("assets/placeholder.png").unwrap(), // normal state image
            tm.get_preload("assets/placeholder.png").unwrap(), // hover state image
        ),
        ImageButton::from_preload(
            420.0,                                             // x position
            410.0,                                             // y position
            100.0,                                             // width
            100.0,                                             // height
            tm.get_preload("assets/placeholder.png").unwrap(), // normal state image
            tm.get_preload("assets/placeholder.png").unwrap(), // hover state image
        ),
    ];
    let mut custom_against = MessageBox::custom(
        "Choose who to play against",
        "Select opponent:",
        vec!["HUMAN", "AI"],
        Some(1), // Default to "Normal"
    );
    let mut custom_diff = MessageBox::custom(
        "Choose Difficulty",
        "Select game difficulty:",
        vec!["Easy", "Normal", "Impossible"],
        Some(1), // Default to "Normal"
    );
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
    let mut ai_first = false;
    let mut ai_dif = 0;
    rand::srand(date::now() as u64);
    let mut started = true;
    let mut ai_bot = false;
    let mut show_custom_against = false;
    loop {
        clear_background(WHITE);
         if ai_bot == true{
     custom_diff.show();

            if let Some(result) = custom_diff.draw() {
                // Only runs when a button was clicked or dialog was closed
                match result {
                    MessageBoxResult::ButtonPressed(0) => {
                        // "Yes" button pressed
                        ai_dif = 1;
                        ai_bot = false;
                    }
                    MessageBoxResult::ButtonPressed(1) => {
                        // "No" button pressed
                        // Continue without saving...
                        ai_dif = 2;
                        ai_bot = false;
                    }
                    MessageBoxResult::ButtonPressed(2) => {
                        // "Cancel" button pressed (for confirm_with_cancel dialogs)
                        // Handle cancel operation...
                        ai_dif = 3;
                        ai_bot = false;
                    }
                    #[allow(unused)]
                    MessageBoxResult::ButtonPressed(_) => {
                        // IMPORTANT: This catch-all pattern is required by the Rust compiler
                        // even for simple confirm dialogs to ensure all possible values are covered
                    }
                    MessageBoxResult::Closed => {
                        // Dialog closed with X or Escape key
                        // Handle as cancel...
                        
                    }
                }
            }         
        }
        if started == true {
            custom_against.show();
            if let Some(result) = custom_against.draw() {
    // Only runs when a button was clicked or dialog was closed
    match result {
        MessageBoxResult::ButtonPressed(0) => {
            // "Yes" button pressed
             ai_bot = false;
             started = false;
        },
        MessageBoxResult::ButtonPressed(1) => {
            // "No" button pressed
            ai_bot = true;
            started = false;
            
        },
        #[allow(unused)]
        MessageBoxResult::ButtonPressed(_) => {
            // IMPORTANT: This catch-all pattern is required by the Rust compiler
            // even for simple confirm dialogs to ensure all possible values are covered
        },
        MessageBoxResult::Closed => {
            // Dialog closed with X or Escape key
            // Handle as cancel...
        }
    }
}  
            
}

        
  

        if game_over == true {
            for i in 0..btns.len() {
                btns[i].enabled = false;
            }
        }
        lbl_score.set_text(&format!("Score: X: {} - {} :O", x_score, o_score));
        for i in (0..btns.len()).step_by(3) {
            if btns[i].get_filename() != "assets/placeholder.png"
                && btns[i].get_filename() == btns[i + 1].get_filename()
                && btns[i].get_filename() == btns[i + 2].get_filename()
                && btns[i + 1].get_filename() == btns[i + 2].get_filename()
                && btns[i].get_filename() == btns[i + 2].get_filename()
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

        for i in (0..3).step_by(1) {
            // dont do 0..btns len cuz then the index cant be greater than 2 before the program crashes as 3 plus 6 is nine and the length of btns is 8 so it crashes
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
                && btns[i].get_filename() == btns[i + 2].get_filename()
                && btns[i].get_filename() == btns[i + 4].get_filename()
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
        } // add reset confirmation to reset score aswell or just gameboard
        if btns[0].get_filename() != "assets/placeholder.png"
            && btns[1].get_filename() != "assets/placeholder.png"
            && btns[2].get_filename() != "assets/placeholder.png"
            && btns[3].get_filename() != "assets/placeholder.png"
            && btns[4].get_filename() != "assets/placeholder.png"
            && btns[5].get_filename() != "assets/placeholder.png"
            && btns[6].get_filename() != "assets/placeholder.png"
            && btns[7].get_filename() != "assets/placeholder.png"
            && btns[8].get_filename() != "assets/placeholder.png"
            && game_over == false
        {
            lbl_Player.set_text("TIE");
        }

        for i in 0..btns.len() {
            if btns[i].click() {
                btns[i].enabled = false;
                if player == false {
                    btns[i].set_preload(tm.get_preload("assets/x.png").unwrap(), tm.get_preload("assets/x.png").unwrap());

                    player = true;
                    // println!("{}", btns[i].get_filename());
                    lbl_Player.set_text("Player: O");
                    if ai_dif>=1{
ai_move(ai_dif, &mut btns, player, &tm, movecount);
                    player = false;
                     lbl_Player.set_text("Player: X");
                     movecount+=1;
                    }
                    
                } else {

                btns[i].set_image("assets/o.png", "assets/o.png").await;
                    player = false;
                lbl_Player.set_text("Player: X");
                }
            }
        }
        if player == true {}
        lbl_score.draw();
        if btn_Exit.click() {
            break;
        }
        if btn_Reset.click() {
            for i in 0..btns.len() {
                btns[i].set_image("assets/placeholder.png", "assets/placeholder.png").await;
                player = false;
                btns[i].enabled = true;
                lbl_Player.set_text("Player: X");
                game_over = false;
                movecount = 1;
            }
        }
        lbl_Player.draw();
        lbl_Title.draw();
        if btn_AI.click() {
custom_diff.show();

            if let Some(result) = custom_diff.draw() {
                // Only runs when a button was clicked or dialog was closed
                match result {
                    MessageBoxResult::ButtonPressed(0) => {
                        // "Yes" button pressed
                        ai_dif = 1;
                        ai_bot = false;
                    }
                    MessageBoxResult::ButtonPressed(1) => {
                        // "No" button pressed
                        // Continue without saving...
                        ai_dif = 2;
                        ai_bot = false;
                    }
                    MessageBoxResult::ButtonPressed(2) => {
                        // "Cancel" button pressed (for confirm_with_cancel dialogs)
                        // Handle cancel operation...
                        ai_dif = 3;
                        ai_bot = false;
                    }
                    #[allow(unused)]
                    MessageBoxResult::ButtonPressed(_) => {
                        // IMPORTANT: This catch-all pattern is required by the Rust compiler
                        // even for simple confirm dialogs to ensure all possible values are covered
                    }
                    MessageBoxResult::Closed => {
                        // Dialog closed with X or Escape key
                        // Handle as cancel...
                        
                    }
                }
            }         
        
            

        }
        if btn_Human.click() {
            ai_dif=0;
        }
        if btn_Easy.click() {
            ai_dif = 1;
        }
        if btn_Medium.click() {
            ai_dif =2;
        }
        if btn_Hard.click() {
            ai_dif = 3;
        }
        custom_against.draw();
        custom_diff.draw();
        next_frame().await;
    }
}
// make animated like just hovering xs and os backgrounmd for the game and a main menu or nah
// make it for the medium ai have it generate a random number 1-3. 1 would make a random move 2 would be a logical move to setup a win and 3 would be a move to hard counter the X's move