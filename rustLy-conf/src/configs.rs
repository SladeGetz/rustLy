extern crate confy;

use std::{fmt, ffi::c_int, path::PathBuf, process::Command, ffi::OsString};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Lang {
    En,
    Fr,
}

#[derive(Debug, Serialize, Deserialize)]
struct Cmd {
    fun: PathBuf,
    args: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    animate: Option<bool>,
    animation: Option<usize>,
    fg: Option<usize>,
    bg: Option<usize>,
    asterisk: Option<char>,
    blank_password: Option<bool>,
    blank_box: Option<bool>,
    hide_borders: Option<bool>,
    margin_box_h: Option<c_int>,
    margin_box_v: Option<c_int>,
    input_len: Option<c_int>,
    max_desktop_len: Option<c_int>,
    max_login_len: Option<c_int>,
    max_password_len: Option<c_int>,
    default_input: Option<c_int>,
    load: Option<bool>,
    save: Option<bool>,
    save_file: Option<PathBuf>,
    hide_fn_commands: Option<bool>,
    shutdown_cmd: Option<Cmd>,
    restart_cmd: Option<Cmd>,
    lang: Option<Lang>,
    tty: Option<c_int>,
    console_dev: Option<PathBuf>,
    default_path: Option<PathBuf>,
    min_refresh_delta: Option<c_int>,
    service_name: Option<String>,
    term_reset_cmd: Option<Cmd>,
    mcookie_cmd: Option<Cmd>,
    wayland_cmd: Option<Cmd>,
    wayland_specifier: Option<bool>,
    waylandsessions: Option<PathBuf>,
    xinitrc: Option<Cmd>,
    x_cmd: Option<Cmd>,
    x_cmd_setup: Option<Cmd>,
    xauth_cmd: Option<Cmd>,
    xsessions: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            animate: Some(false),
            animation: Some(0),
            fg: Some(1),
            bg: Some(0),
            asterisk: Some('*'),
            blank_password: Some(false),
            blank_box: Some(true),
            hide_borders: Some(false),
            margin_box_h: Some(2),
            margin_box_v: Some(1),
            input_len: Some(34),
            max_desktop_len: Some(100),
            max_login_len: Some(255),
            max_password_len: Some(255),
            default_input: Some(2),
            load: Some(true),
            save: Some(true),
            save_file: Some(PathBuf::from(r"/etc/rustLy/save/")),
            hide_fn_commands: Some(false),
            shutdown_cmd: Some( Cmd { 
                fun: PathBuf::from(r"/sbin/shutdown"),
                args: Some(vec![String::from("-a"), String::from("now")]),
                }),
            restart_cmd: Some( Cmd {
                fun: PathBuf::from(r"/sbin/shutdown"),
                args: Some(vec![String::from("-r"), String::from("now")]),
                }),
            lang: Some(Lang::En),
            tty: Some(2),
            console_dev: Some(PathBuf::from(r"/dev/console")),
            default_path: Some(PathBuf::from(r"/sbin:/bin:/usr/local/sbin:/usr/local/bin:/usr/bin:/usr/sbin")),
            min_refresh_delta: Some(5),
            service_name: Some(String::from("rustLy")),
            term_reset_cmd: Some( Cmd {
                fun: PathBuf::from(r"/usr/bin/tput"),
                args: Some(vec![String::from("reset")]),
                }),
            mcookie_cmd: Some( Cmd {
                fun: PathBuf::from(r"/usr/bin/mcookie"),
                args: None,
                }),
            wayland_cmd: Some( Cmd {
                fun: PathBuf::from(r"/etc/rustLy/wsetup.sh"),
                args: None,
                }),
            wayland_specifier: Some(false),
            waylandsessions: Some(PathBuf::from(r"/usr/share/wayland-sessions")),
            xinitrc: Some( Cmd {
                fun: PathBuf::from(r"~/.xinitrc"),
                args: None,
                }),
            x_cmd: Some( Cmd {
                fun: PathBuf::from(r"/usr/bin/X"),
                args: None,
                }),
            x_cmd_setup: Some( Cmd {
                fun: PathBuf::from(r"/etc/rustLy/xsetup.sh"),
                args: None,
                }),
            xauth_cmd: Some( Cmd {
                fun: PathBuf::from(r"/usr/bin/xauth"),
                args: None,
                }),
            xsessions: Some(PathBuf::from(r"/usr/share/xsessions")),
        }
    }
}

impl Config {
    pub fn animate(&self) -> bool {
        self.animate.unwrap_or_default()
    }
    pub fn animation(&self) -> usize {
        self.animation.unwrap_or_default()
    }
    pub fn fg(&self) -> usize {
        self.fg.unwrap_or_default()
    }
    pub fn bg(&self) -> usize {
        self.bg.unwrap_or_default()
    }
    pub fn asterisk(&self) -> char {
        self.asterisk.unwrap_or_default()
    }
    pub fn blank_password(&self) -> bool {
        self.blank_password.unwrap_or_default()
    }
    pub fn blank_box(&self) -> bool {
        self.blank_box.unwrap_or_default()
    }
    pub fn hide_borders(&self) -> bool {
        self.hide_borders.unwrap_or_default()
    }
    pub fn margin_box_v(&self) -> c_int {
        self.margin_box_v.unwrap_or_default()
    }
    pub fn margin_box_h(&self) -> c_int {
        self.margin_box_h.unwrap_or_default()
    }
    pub fn input_len(&self) -> c_int {
        self.input_len.unwrap_or_default()
    }
}

// impl fmt::Display for Config {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//         // f.write_str("Config:\n\tanimate: ")?;
//         // f.write_str(&self.animate.to_string())?;
//         // f.write_str("\n\tanimation: ")?;
//         // f.write_str(&self.animation.to_string())?;
//         // f.write_str("{}\n\tfg: ")?;
//         // f.write_str(&self.fg.to_string())?;
//         // f.write_str("\n\tbg: ")?;
//         // f.write_str(&self.bg.to_string())?;
//         // f.write_str("\n")?;
//         // Ok(())
//     }
// }




fn error_message(err: confy::ConfyError) -> Config {
    println!("Problem with config: {:?}\n using defaults", err);
    let conf = Config { ..Default::default() };
    return conf;
}

pub fn get_configs() -> Config {
    let cfg_result: Result<Config, confy::ConfyError> = confy::load("rustLy", "config");

    let cfg = match cfg_result {
        Ok(c) => c,
        Err(err) => error_message(err),
    };

    // println!("{:?}", cfg);
    // println!("FG: {:?}", cfg.fg());
    // println!("Margin Box: {:?}", cfg.margin_box_v());

    return cfg;
}
