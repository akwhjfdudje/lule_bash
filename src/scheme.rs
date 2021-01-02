#![allow(dead_code)]

extern crate getset;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use std::collections::HashMap as Map;

#[derive(Clone, CopyGetters, Getters, MutGetters, Setters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
pub struct WRITE {
    wallpaper: String,
    theme: String,
    colors: Vec<pastel::Color>,
}

impl WRITE {
    pub fn new(wallpaper: String, theme: String, colors: Vec<pastel::Color>) -> Self {
        Self {
            wallpaper,
            theme,
            colors,
        }
    }
    pub fn init() -> Self {
        Self {
            wallpaper: String::new(),
            theme: String::new(),
            colors: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, CopyGetters, Getters, MutGetters, Setters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
pub struct SCHEME {
    colors: Option<Vec<String>>,
    image: Option<String>,
    scheme: Option<String>,
    walldir: Option<String>,
    config: Option<String>,
    cache: Option<String>,
    script: Option<String>,
    looop: Option<usize>,
    theme: Option<String>,
    palette: Option<String>,
    sort: Option<String>,
    saturation: Option<f32>,
    illumination: Option<f32>,
    hue: Option<f32>,
    difference: Option<f32>,
    blend: Option<f32>,
    mixes: Option<Map<usize, String>>,
}

impl SCHEME {
    pub fn init() -> Self {
        Self {
            colors: None,
            image: None,
            scheme: None,
            walldir: None,
            config: None,
            cache: None,
            script: None,
            looop: None,
            theme: None,
            palette: None,
            sort: None,
            saturation: None,
            illumination: None,
            hue: None,
            difference: None,
            blend: None,
            mixes: None 
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, CopyGetters, Getters, MutGetters, Setters)]
pub struct Special {
    pub background: String,
    pub foreground: String,
    pub cursor: String
}

#[derive(Serialize, Deserialize, Debug, Clone, CopyGetters, Getters, MutGetters, Setters)]
pub struct ProfileMap {
    pub wallpaper: String,
    pub theme: String,
    pub special: Special,
    pub colors: Map<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, CopyGetters, Getters, MutGetters, Setters)]
pub struct ProfileVec {
    pub wallpaper: String,
    pub theme: String,
    pub special: Special,
    pub colors: Vec<String>,
}
