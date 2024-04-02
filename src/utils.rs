use mouse_position::mouse_position::Mouse;
use std::thread;
use std::time::Duration;

pub fn detect_mouse_position(wait_before_start: u64) -> Vec<[i32; 4]> {
    let mut num = 0;
    let mut first_pos_x = 0;
    let mut first_pos_y = 0;
    let mut pos_arr: Vec<[i32; 4]> = Vec::new();
    loop {
        num += 1;
        if num > 6 {
            break;
        }
        println!(
            "start detect the cursor position in {} seconds...",
            wait_before_start / 1000
        );
        sleep(wait_before_start);
        let position = Mouse::get_mouse_position();
        match position {
            Mouse::Position { x, y } => {
                let delta_x = if num == 1 { 0 } else { x - first_pos_x };
                let delta_y = if num == 1 { 0 } else { y - first_pos_y };
                println!(
                    "mouse position {} : -x {} -y {} delta_x {} delta_y {}",
                    num, x, y, delta_x, delta_y,
                );
                pos_arr.push([x, y, delta_x, delta_y]);

                if num == 1 {
                    (first_pos_x, first_pos_y) = (x, y);
                }
            }
            Mouse::Error => println!("could not detect the mouse position!"),
        }
    }
    pos_arr
}

pub fn sleep(duration: u64) {
    thread::sleep(Duration::from_millis(duration));
}
