use serde::{Deserialize, Serialize};
use serde_json::{json, value};
use std::fs;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameData {
    pub name: String,
    pub version: String,
    pub api: String,
    pub categories: Vec<Category>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Settings {
    pub game_type: bool
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Category {
    pub name: String,
    pub slug: String,
    pub cover: File,
    pub levels: Vec<Level>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Level {
    pub name: String,
    pub slug: String,
    pub cover: File,
    pub background: File,
    pub pieces: Vec<Piece>
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct File {
    pub route: String,
    pub filename: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Piece {
    pub size: Size,
    pub position: Position,
    pub img: File,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl GameData {
    pub fn new() -> Self {
        let path = std::path::Path::new("static/data.json");
        let file = fs::read_to_string(path).unwrap();
        let game_data: GameData = serde_json::from_str(&file).unwrap();
        println!("GameData: {:?}", game_data);
        game_data
    }
}

impl Default for GameData {
    fn default() -> Self {
        GameData {
            name: "default".to_string(),
            version: "0.0.0".to_string(),
            api: "".to_string(),
            categories: vec![
                Category {
                    name: "".to_string(),
                    slug: "".to_string(),
                    cover: File {
                        route: "".to_string(),
                        filename: "".to_string()
                    },
                    levels: vec![
                        Level {
                            name: "".to_string(),
                            slug: "".to_string(),
                            cover: File { route: "".to_string(), filename: "".to_string() },
                            background: File { route: "".to_string(), filename: "".to_string() },
                            pieces: vec![
                                Piece {
                                    size: Size { width: 0, height: 0 },
                                    position: Position { x: 0, y: 0 },
                                    img: File { route: "".to_string(), filename: "".to_string() }
                                }
                            ]
                        }
                    ]
                    
                }
            ]
        }
    }
}