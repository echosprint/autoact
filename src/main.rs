mod config;
mod execute;
mod utils;

use clap::Parser;
use config::{config, write_action_snippet, write_sample_config};
use execute::execute_action;
use utils::detect_mouse_position;

// Automatic action
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// the config file path
    #[arg(short, long, default_value_t = String::new())]
    config: String,

    /// detect the current cursor position
    #[arg(short, long, action)]
    detect: bool,

    /// execute the actions in the config file
    #[arg(short, long, action)]
    execute: bool,

    /// generate the sample config toml file
    #[arg(short, long, action)]
    sample: bool,

    /// set initial x cursor
    #[arg(short, long, default_value_t = -1)]
    x: i32,

    /// set initial y cursor
    #[arg(short, long, default_value_t = -1)]
    y: i32,
}

fn main() {
    let args = Args::parse();
    let action_config = config(&args.config);
    // println!("{:#?}", action_config);

    if args.detect {
        let pos_arr = detect_mouse_position(action_config.wait_before_detect_cursor);
        write_action_snippet(pos_arr);
        return;
    }

    if args.sample {
        write_sample_config();
        return;
    }

    if args.execute || action_config.execute_action {
        execute_action(&action_config.action, args.x, args.y);
    } else {
        println!(
            "action in config file not executed, please set -e in CLI or set execute_action = true in config file"
        );
    }
}
