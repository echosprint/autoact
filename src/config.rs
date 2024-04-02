use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::read_to_string;
use std::io::Write;
use std::path::PathBuf;
use std::process;

const TOML_FILE: &str = "action_config.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Action {
    SETORIGIN {
        x: i32,
        y: i32,
    },
    UNSETORIGIN,
    MOVEINIT {
        #[serde(default = "default_wait")]
        wait: u64,
    },
    MOVE {
        x: i32,
        y: i32,
        #[serde(default = "default_wait")]
        wait: u64,
        #[serde(default = "default_relative")]
        relative: bool,
    },
    MOVECLICK {
        x: i32,
        y: i32,
        #[serde(default = "default_wait")]
        wait: u64,
        #[serde(default = "default_relative")]
        relative: bool,
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
    CHANGE {
        x: i32,
        y: i32,
        content: String,
        #[serde(default = "default_delnum")]
        delnum: i32,
        #[serde(default = "default_wait")]
        wait: u64,
        #[serde(default = "default_relative")]
        relative: bool,
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
    #[serde(default = "default_wait_execute")]
    pub execute_action: bool,
    #[serde(default = "default_wait_detect")]
    pub wait_before_detect_cursor: u64,
    pub action: Vec<Action>,
}

fn default_wait() -> u64 {
    500
}

fn default_wait_detect() -> u64 {
    5_000
}

fn default_delnum() -> i32 {
    -1
}

fn default_relative() -> bool {
    false
}

fn default_wait_execute() -> bool {
    true
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

pub fn write_sample_config() {
    let exe_path: PathBuf = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let mut file = fs::File::create(exe_dir.join("sample_config.toml")).unwrap();
    file.write_all(TOML_SAMPLE.as_bytes()).unwrap();
}

pub fn write_action_snippet(pos_arr: Vec<[i32; 4]>) {
    let exe_path: PathBuf = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let mut file = fs::File::create(exe_dir.join("action_snippet.toml")).unwrap();
    let formatted_strings: Vec<String> = pos_arr.iter().enumerate()
        .map(|(idx, array)| format!("# position {}\n[[action]]\ntype = \"MOVECLICK\"\nx = {}\ny = {}\n\n[[action]]\ntype = \"MOVECLICK\"\nx = {}\ny = {}\nrelative = true\n",
         idx+1, array[0], array[1], array[2], array[3]))
        .collect();

    // Concatenating all formatted strings with "\n" as a separator
    let result = formatted_strings.join("\n");
    file.write_all(result.as_bytes()).unwrap();
}

const TOML_SAMPLE: &str = r###"
execute_action = true # optional, default true
wait_before_detect_cursor = 4_000 # optional, ms


# set the origin, so that later action can move relative to ORIGIN
[[action]]
type = "SETORIGIN"
x = 100
y = 100

# unset the origin to (0, 0)
[[action]]
type = "UNSETORIGIN"

# move to initial position specified in the CLI parameters use -x -y
[[action]]
type = "MOVEINIT"
wait = 100 # optional, ms

# move the mouse cursor to specified x, y cordinates
[[action]]
type = "MOVE"
x = 100
y = 100
wait = 100 # optional, ms
relative = false # optional, if true, move to x, y relative to the origin

# move the mouse cursor to specified x, y coordinates, then click left button
[[action]]
type = "MOVECLICK"
x = 100
y = 100
wait = 100 # optional, ms
relative = false # optional, if true, move to x, y relative to the origin

# move relative to last position
[[action]]
type = "MOVERELATIVE"
x = 100
y = 100
wait = 100 # optional, ms

# click the left mouse button
[[action]]
type = "LEFTCLICK"
wait = 1_000 # optional, ms

# double click the left mouse button
[[action]]
type = "DOUBLECLICK"
wait = 1_000 # optional, ms

# click the right mouse button
[[action]]
type = "RIGHTCLICK"
wait = 2_000 # optional, ms

# wait for specified time, DO NOTHING
[[action]]
type = "WAIT"
wait = 5_000 # optional, ms

# press the BACKSPACE key for number times
[[action]]
type = "BACKSPACE"
number = 8
wait = 300 # optional, ms

# press the DELETE key
[[action]]
type = "DELETE"
wait = 100 # optional, ms

# input a string
[[action]]
type = "INPUT"
content = "Hello world"
wait = 100 # optional, ms

# change the content at specified x, y coordinates
# move to (x, y), then click, the press BACKSPACE delnum times, then input the content
[[action]]
type = "CHANGE"
x = 100
y = 100
content = "hello, world"
delnum = 10 # press BACKSPACE times
wait = 100
relative = true # if true, relative to origin

# input DATE as content, format like `20240331`
[[action]]
type = "DATE"
wait = 100

# press the ENTER key
[[action]]
type = "ENTER"
"###;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_config() {
        let config: Config = toml::from_str(TOML_SAMPLE).unwrap();

        println!("{:#?}", config);
    }
}
