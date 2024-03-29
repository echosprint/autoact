use serde::{Deserialize, Serialize};
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process;

const TOML_FILE: &str = "action_config.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Action {
    MOVEINIT {
        #[serde(default = "default_wait")]
        wait: u64,
    },
    MOVE {
        x: i32,
        y: i32,
        #[serde(default = "default_wait")]
        wait: u64,
    },
    MOVECLICK {
        x: i32,
        y: i32,
        #[serde(default = "default_wait")]
        wait: u64,
    },
    MOVERELATIVE {
        x: i32,
        y: i32,
        #[serde(default = "default_wait")]
        wait: u64,
    },
    LEFTCLICK {
        #[serde(default = "default_wait")]
        wait: u64,
    },
    DOUBLECLICK {
        #[serde(default = "default_wait")]
        wait: u64,
    },
    RIGHTCLICK {
        #[serde(default = "default_wait")]
        wait: u64,
    },
    WAIT {
        #[serde(default = "default_wait")]
        wait: u64,
    },

    BACKSPACE {
        number: i32,
        #[serde(default = "default_wait")]
        wait: u64,
    },

    DELETE {
        #[serde(default = "default_wait")]
        wait: u64,
    },

    INPUT {
        content: String,
        #[serde(default = "default_wait")]
        wait: u64,
    },

    DATE {
        #[serde(default = "default_wait")]
        wait: u64,
    },

    ENTER {
        #[serde(default = "default_wait")]
        wait: u64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub execute_action: bool,
    pub wait_before_detect_cursor: u64,
    pub action: Vec<Action>,
}

fn default_wait() -> u64 {
    500
}

pub fn config(config_file: &str) -> Config {
    let exe_path: PathBuf = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let mut file_path = config_file.to_string();
    if config_file.len() <= 0 {
        file_path = exe_dir.join(TOML_FILE).to_string_lossy().to_string();
    }
    let toml_str = match read_to_string(&file_path) {
        Ok(toml_string) => toml_string,
        Err(_) => {
            println!("config file: {} not found", &file_path);
            process::exit(1);
        }
    };
    let config: Config =
        toml::from_str(&toml_str).expect(&format!("cannot parse the {} file", file_path));

    config
}
