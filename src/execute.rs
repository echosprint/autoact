use crate::config::Action;
use crate::utils::sleep;
use chrono::{Datelike, Local};
use enigo::*;
use std::process;

pub fn execute_action(actions: &[Action], init_x: i32, init_y: i32) {
    let mut enigo = Enigo::new();
    let mut origin_x = 0;
    let mut origin_y = 0;

    for action in actions {
        let act = action.clone();
        match act {
            Action::SETORIGIN { x, y } => {
                origin_x = x;
                origin_y = y
            }
            Action::UNSETORIGIN => {
                origin_x = 0;
                origin_y = 0;
            }
            Action::MOVEINIT { wait } => {
                if init_x < 0 || init_y < 0 {
                    println!("Please specify the initial cursor -x -y in CLI parameters, actions not executed");
                    process::exit(1);
                }
                println!(
                    "move mouse cursor to initial position x:{} y:{}",
                    init_x, init_y
                );
                enigo.mouse_move_to(init_x, init_y);
                println!("sleep for {}ms", wait);
                sleep(wait);
            }
            Action::MOVE {
                x,
                y,
                wait,
                relative,
            } => {
                let (x, y) = if relative {
                    (x + origin_x, y + origin_y)
                } else {
                    (x, y)
                };
                println!("move mouse cursor to position x:{} y:{}", x, y);
                enigo.mouse_move_to(x, y);
                println!("sleep for {}ms", wait);
                sleep(wait);
            }
            Action::MOVECLICK {
                x,
                y,
                wait,
                relative,
            } => {
                let (x, y) = if relative {
                    (x + origin_x, y + origin_y)
                } else {
                    (x, y)
                };
                println!("move mouse cursor to position x:{} y:{}, and click", x, y);
                enigo.mouse_move_to(x, y);
                sleep(10);
                enigo.mouse_click(MouseButton::Left);
                println!("sleep for {}ms", wait);
                sleep(wait);
            }
            Action::MOVERELATIVE { x, y, wait } => {
                println!("move mouse cursor relatively x:{} y:{}", x, y);
                enigo.mouse_move_relative(x, y);
                println!("sleep for {}ms", wait);
                sleep(wait);
            }
            Action::LEFTCLICK { wait } => {
                println!("click the left mouse button");
                enigo.mouse_click(MouseButton::Left);
                println!("sleep for {}ms", wait);
                sleep(wait);
            }
            Action::DOUBLECLICK { wait } => {
                println!("double click the left mouse button");
                enigo.mouse_click(MouseButton::Left);
                enigo.mouse_click(MouseButton::Left);
                println!("sleep for {}ms", wait);
                sleep(wait);
            }
            Action::RIGHTCLICK { wait } => {
                println!("click the right mouse button");
                enigo.mouse_click(MouseButton::Right);
                println!("sleep for {}ms", wait);
                sleep(wait);
            }
            Action::WAIT { wait } => {
                println!("sleep for {}ms", wait);
                sleep(wait);
            }

            Action::BACKSPACE { number, wait } => {
                println!("press the BACKSPACE {} times", number);
                (0..number).for_each(|_| enigo.key_click(Key::Backspace));
                println!("sleep for {}ms", wait);
                sleep(wait);
            }

            Action::DELETE { wait } => {
                println!("press the DELETE key");
                enigo.key_click(Key::Delete);
                println!("sleep for {}ms", wait);
                sleep(wait);
            }

            Action::INPUT { content, wait } => {
                println!("input the content: {}", content);
                enigo.key_sequence(&content);
                println!("sleep for {}ms", wait);
                sleep(wait);
            }

            Action::CHANGE {
                x,
                y,
                content,
                delnum,
                wait,
                relative,
            } => {
                let (x, y) = if relative {
                    (x + origin_x, y + origin_y)
                } else {
                    (x, y)
                };

                let parsed_content = if content == "${DATE}" {
                    get_date_str()
                } else {
                    content
                };

                let number = if delnum < 0 {
                    parsed_content.len()
                } else {
                    delnum as usize
                };

                println!(
                    "move to x:{} y:{}, change the content to: {}",
                    x, y, parsed_content
                );

                enigo.mouse_move_to(x, y);
                sleep(10);
                enigo.mouse_click(MouseButton::Left);
                sleep(50);
                (0..number).for_each(|_| enigo.key_click(Key::Backspace));
                sleep(300);
                enigo.key_sequence(&parsed_content);
                println!("sleep for {}ms", wait);
                sleep(wait);
            }

            Action::DATE { wait } => {
                let formatted_date = get_date_str();
                println!("input the date string: {}", &formatted_date);
                enigo.key_sequence(&formatted_date);
                println!("sleep for {}ms", wait);
                sleep(wait);
            }

            Action::ENTER { wait } => {
                println!("press the RETURN key");
                enigo.key_click(Key::Return);
                println!("sleep for {}ms", wait);
                sleep(wait);
            }
        }
    }
}

fn get_date_str() -> String {
    let now = Local::now(); // Get the current date and time
    let formatted_date = format!("{:04}{:02}{:02}", now.year(), now.month(), now.day());
    formatted_date
}
