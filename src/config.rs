use crate::consts;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Keymap {
    pub version: u8,
    pub keyboard: String,
    pub keymap: String,
    pub layout: String,
    pub layers: Vec<Vec<String>>,
}

impl Keymap {
    pub fn new(keyboard: String, keymap: Vec<Vec<u8>>) -> Self {
        Self {
            version: 1,
            keyboard,
            keymap: "default".to_string(),
            layout: "LAYOUT".to_string(),
            layers: layers_from_keymap(keymap),
        }
    }
    pub fn encode(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
    pub fn decode(keymap: &str) -> serde_json::Result<Self> {
        serde_json::from_str(keymap)
    }
}

pub fn layers_from_keymap(keymap: Vec<Vec<u8>>) -> Vec<Vec<String>> {
    keymap
        .iter()
        .map(|layer| {
            layer
                .iter()
                .map(|&keycode| consts::key_code_name(keycode as usize))
                .collect()
        })
        .collect()
}

pub fn keymap_from_layers(layers: Vec<Vec<String>>) -> Vec<Vec<u8>> {
    layers
        .iter()
        .map(|layer| {
            layer
                .iter()
                .map(|name| consts::key_code_from_name(name).unwrap_or(0) as u8)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every keycode the firmware can report has to survive a pull/push round trip,
    /// otherwise editing one key rewrites the ones the table has no name for.
    #[test]
    fn keymap_round_trip_is_lossless() {
        let keymap: Vec<Vec<u8>> = (0..=255u8).map(|keycode| vec![keycode]).collect();
        assert_eq!(keymap_from_layers(layers_from_keymap(keymap.clone())), keymap);
    }
}
