extern crate confy;

use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    animate: bool,
    animation: usize,
    fg: usize,
    bg: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            animate: false,
            animation: 0,
            fg: 1,
            bg: 0,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Config:\n\tanimate: ")?;
        f.write_str(&self.animate.to_string())?;
        f.write_str("\n\tanimation: ")?;
        f.write_str(&self.animation.to_string())?;
        f.write_str("{}\n\tfg: ")?;
        f.write_str(&self.fg.to_string())?;
        f.write_str("\n\tbg: ")?;
        f.write_str(&self.bg.to_string())?;
        f.write_str("\n")?;
        Ok(())
    }
}

impl Config {
    pub fn animate(&self) -> bool {
        self.animate
    }
    pub fn animation(&self) -> usize {
        self.animation
    }
    pub fn fg(&self) -> usize {
        self.fg
    }
    pub fn bg(&self) -> usize {
        self.bg
    }
}


fn errorMessage(err: confy::ConfyError) -> Config {
    println!("Problem with config: {:?}\n using defaults", err);
    let conf = Config { ..Default::default() };
    return conf;
}

pub fn getConfigs() -> Config {
    let cfg_result: Result<Config, confy::ConfyError> = confy::load("rustLy", "config");

    let cfg = match cfg_result {
        Ok(c) => c,
        Err(err) => errorMessage(err),
    };

    return cfg;
}
