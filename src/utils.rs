use mouse_position::mouse_position::Mouse;
use std::thread;
use std::time::Duration;

pub fn detect_mouse_position(wait_before_start: u64) {
    println!(
        "start detect the cursor position in {} seconds...",
        wait_before_start / 1000
    );
    sleep(wait_before_start);
    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => {
            println!("mouse position: -x {} -y {}", x, y);
        }
        Mouse::Error => println!("could not detect the mouse position!"),
    }
}

pub fn sleep(duration: u64) {
    thread::sleep(Duration::from_millis(duration));
}
