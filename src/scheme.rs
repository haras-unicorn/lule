use getset::{CopyGetters, Getters, MutGetters, Setters};
use std::collections::HashMap as Map;

#[derive(Serialize, Deserialize, Debug, Clone, CopyGetters, Getters, MutGetters, Setters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
pub struct Scheme {
    #[serde(skip)]
    colors: Option<Vec<pastel::Color>>,
    image: Option<String>,
    theme: Option<String>,
    pigments: Option<Vec<String>>,
    scheme: Option<String>,
    walldir: Option<String>,
    #[serde(skip)]
    config: Option<String>,
    #[serde(skip)]
    cache: Option<String>,
    scripts: Option<Vec<String>>,
    patterns: Option<Vec<(String, String)>>,
    looop: Option<usize>,
    palette: Option<String>,
    sort: Option<String>,
    saturation: Option<f32>,
    illumination: Option<f32>,
    hue: Option<f32>,
    difference: Option<f32>,
    blend: Option<f32>,
    mixes: Option<Map<usize, String>>,
}

impl Scheme {
    pub fn init() -> Self {
        Self {
            colors: None,
            pigments: None,
            image: None,
            scheme: None,
            walldir: None,
            config: None,
            cache: None,
            scripts: None,
            patterns: None,
            looop: None,
            theme: None,
            palette: None,
            sort: None,
            saturation: None,
            illumination: None,
            hue: None,
            difference: None,
            blend: None,
            mixes: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, CopyGetters, Getters, MutGetters, Setters)]
pub struct Special {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
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
