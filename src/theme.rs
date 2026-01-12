use gdk4::RGBA;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub palette: Vec<String>,
}

impl Theme {
    pub fn background_rgba(&self) -> RGBA {
        parse_color(&self.background)
    }

    pub fn foreground_rgba(&self) -> RGBA {
        parse_color(&self.foreground)
    }

    pub fn cursor_rgba(&self) -> RGBA {
        parse_color(&self.cursor)
    }

    pub fn palette_rgba(&self) -> Vec<RGBA> {
        self.palette.iter().map(|c| parse_color(c)).collect()
    }
}

fn parse_color(hex: &str) -> RGBA {
    RGBA::parse(hex).unwrap_or_else(|_| RGBA::new(1.0, 1.0, 1.0, 1.0))
}

pub fn get_themes() -> Vec<Theme> {
    vec![
        Theme {
            name: "default".to_string(),
            background: "#1e1e2e".to_string(),
            foreground: "#cdd6f4".to_string(),
            cursor: "#f5e0dc".to_string(),
            palette: vec![
                "#45475a".to_string(), // black
                "#f38ba8".to_string(), // red
                "#a6e3a1".to_string(), // green
                "#f9e2af".to_string(), // yellow
                "#89b4fa".to_string(), // blue
                "#f5c2e7".to_string(), // magenta
                "#94e2d5".to_string(), // cyan
                "#bac2de".to_string(), // white
                "#585b70".to_string(), // bright black
                "#f38ba8".to_string(), // bright red
                "#a6e3a1".to_string(), // bright green
                "#f9e2af".to_string(), // bright yellow
                "#89b4fa".to_string(), // bright blue
                "#f5c2e7".to_string(), // bright magenta
                "#94e2d5".to_string(), // bright cyan
                "#a6adc8".to_string(), // bright white
            ],
        },
        Theme {
            name: "light".to_string(),
            background: "#eff1f5".to_string(),
            foreground: "#4c4f69".to_string(),
            cursor: "#dc8a78".to_string(),
            palette: vec![
                "#5c5f77".to_string(),
                "#d20f39".to_string(),
                "#40a02b".to_string(),
                "#df8e1d".to_string(),
                "#1e66f5".to_string(),
                "#ea76cb".to_string(),
                "#179299".to_string(),
                "#acb0be".to_string(),
                "#6c6f85".to_string(),
                "#d20f39".to_string(),
                "#40a02b".to_string(),
                "#df8e1d".to_string(),
                "#1e66f5".to_string(),
                "#ea76cb".to_string(),
                "#179299".to_string(),
                "#bcc0cc".to_string(),
            ],
        },
        Theme {
            name: "solarized_dark".to_string(),
            background: "#002b36".to_string(),
            foreground: "#839496".to_string(),
            cursor: "#93a1a1".to_string(),
            palette: vec![
                "#073642".to_string(),
                "#dc322f".to_string(),
                "#859900".to_string(),
                "#b58900".to_string(),
                "#268bd2".to_string(),
                "#d33682".to_string(),
                "#2aa198".to_string(),
                "#eee8d5".to_string(),
                "#002b36".to_string(),
                "#cb4b16".to_string(),
                "#586e75".to_string(),
                "#657b83".to_string(),
                "#839496".to_string(),
                "#6c71c4".to_string(),
                "#93a1a1".to_string(),
                "#fdf6e3".to_string(),
            ],
        },
        Theme {
            name: "dracula".to_string(),
            background: "#282a36".to_string(),
            foreground: "#f8f8f2".to_string(),
            cursor: "#f8f8f2".to_string(),
            palette: vec![
                "#21222c".to_string(),
                "#ff5555".to_string(),
                "#50fa7b".to_string(),
                "#f1fa8c".to_string(),
                "#bd93f9".to_string(),
                "#ff79c6".to_string(),
                "#8be9fd".to_string(),
                "#f8f8f2".to_string(),
                "#6272a4".to_string(),
                "#ff6e6e".to_string(),
                "#69ff94".to_string(),
                "#ffffa5".to_string(),
                "#d6acff".to_string(),
                "#ff92df".to_string(),
                "#a4ffff".to_string(),
                "#ffffff".to_string(),
            ],
        },
    ]
}

pub fn get_theme_by_name(name: &str) -> Theme {
    get_themes()
        .into_iter()
        .find(|t| t.name == name)
        .unwrap_or_else(|| get_themes().into_iter().next().unwrap())
}
