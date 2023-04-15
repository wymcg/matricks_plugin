use serde::{Serialize, Deserialize};

/// Matrix configuration information to be passed to the plugin's setup function
#[derive(Serialize, Deserialize, Clone)]
pub struct MatrixConfiguration {
    /// Width of the matrix, in number of LEDs
    pub width: usize,

    /// Height of the matrix, in number of LEDs
    pub height: usize,

    /// FPS that the program will attempt to drive the LEDs
    pub target_fps: f32,

    /// Data line alternates direction between columns or rows
    /// In other words, every other row or column is reversed
    pub serpentine: bool,

    #[cfg(not(target_arch = "aarch64"))]
    /// Magnification of the simulated matrix
    pub magnification: f32,
}

impl Default for MatrixConfiguration {
    fn default() -> Self {
        Self {
            width: 0, 
            height: 0,
            target_fps: 0.0,
            serpentine: false,
            magnification: 0.0,
        }
    }
}

/// Update retrieved from the plugin every call to the update function
#[derive(Serialize, Deserialize, Clone)]
pub struct PluginUpdate {
    /// State of each LED in the matrix, as a two-dimensional matrix of BGRA values
    pub state: Vec<Vec<[u8; 4]>>,

    /// Whether or not the plugin is done providing updates.
    ///
    /// If this is ever set to true, the main thread will move on to the next available plugin
    pub done: bool,

    /// Logs made by the plugin
    ///
    /// If this is not None, the main thread will log the strings in the list on behalf of the plugin
    pub log_message: Option<Vec<String>>,
}

impl Default for PluginUpdate {
    fn default() -> Self {
        Self {
            state: vec![],
            done: false,
            log_message: None,
        }
    }
}
