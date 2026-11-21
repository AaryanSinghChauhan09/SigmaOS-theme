// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-theme — Wallust-style dynamic theme engine.
//!
//! Color model, built-in presets (Catppuccin/Tokyo Night/Nord/…), palette
//! application that renders CSS variables + hyprland colors, and theme
//! registry/cycling. Deterministic; a wallpaper-sampling backend (Wallust)
//! can replace preset selection later.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }

    /// Parse "#rrggbb" or "rrggbb".
    pub fn from_hex(s: &str) -> Result<Self, ThemeError> {
        let h = s.trim().trim_start_matches('#');
        if h.len() != 6 { return Err(ThemeError::BadHex(s.into())); }
        let v = u32::from_str_radix(h, 16).map_err(|_| ThemeError::BadHex(s.into()))?;
        Ok(Self::rgb(((v >> 16) & 0xff) as u8, ((v >> 8) & 0xff) as u8, (v & 0xff) as u8))
    }

    pub fn hex(&self) -> String { format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b) }

    /// Approximate xterm-256 color index (6x6x6 cube + grayscale fallback).
    pub fn ansi256(&self) -> u8 {
        let ri = (self.r as u16 * 5 / 255) as u8;
        let gi = (self.g as u16 * 5 / 255) as u8;
        let bi = (self.b as u16 * 5 / 255) as u8;
        16 + 36 * ri + 6 * gi + bi
    }

    /// Relative luminance 0.0–1.0 (for contrast decisions).
    pub fn luminance(&self) -> f32 {
        let f = |c: u8| { let c = c as f32 / 255.0; if c <= 0.03928 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) } };
        0.2126 * f(self.r) + 0.7152 * f(self.g) + 0.0722 * f(self.b)
    }
    pub fn is_light(&self) -> bool { self.luminance() > 0.5 }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.hex()) }
}

/// Named palette slots every theme must provide.
pub const PALETTE_KEYS: &[&str] = &[
    "background", "foreground", "cursor", "selection",
    "primary", "secondary", "accent", "error", "warning", "success", "info",
    "surface0", "surface1", "surface2", "overlay0",
];

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub variant: Variant,
    pub palette: BTreeMap<String, Color>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant { Dark, Light }

impl Variant {
    pub fn label(self) -> &'static str { match self { Self::Dark => "dark", Self::Light => "light" } }
}

impl Theme {
    pub fn get(&self, key: &str) -> Option<Color> { self.palette.get(key).copied() }
    pub fn is_complete(&self) -> bool { PALETTE_KEYS.iter().all(|k| self.palette.contains_key(*k)) }

    /// Render CSS custom properties (for waybar/rofi/gtk CSS).
    pub fn css_vars(&self) -> String {
        let mut s = String::from(":root {\n");
        for (k, c) in &self.palette {
            s.push_str(&format!("  --{k}: {};\n", c.hex()));
        }
        s.push_str("}\n");
        s
    }

    /// Render hyprland `col.*` color env lines.
    pub fn hyprland_colors(&self) -> String {
        let mut s = String::new();
        for (k, c) in &self.palette {
            s.push_str(&format!("col.{k} = {}\n", c.hex()));
        }
        s
    }

    /// 16-color terminal ANSI palette derived from theme slots.
    pub fn terminal_colors(&self) -> Vec<u8> {
        let pick = |k: &str, fb: Color| self.get(k).unwrap_or(fb);
        [
            pick("background", Color::rgb(0,0,0)), pick("surface0", Color::rgb(30,30,30)),
            pick("success", Color::rgb(80,200,120)), pick("warning", Color::rgb(220,180,60)),
            pick("error", Color::rgb(220,80,80)), pick("accent", Color::rgb(100,140,255)),
            pick("info", Color::rgb(80,180,220)), pick("overlay0", Color::rgb(120,120,120)),
            pick("foreground", Color::rgb(200,200,200)), pick("surface1", Color::rgb(60,60,60)),
            pick("surface2", Color::rgb(90,90,90)), pick("secondary", Color::rgb(140,140,140)),
            pick("primary", Color::rgb(180,180,180)), pick("selection", Color::rgb(60,60,120)),
            pick("cursor", Color::rgb(255,255,255)), pick("background", Color::rgb(0,0,0)),
        ].iter().map(|c| c.ansi256()).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeError {
    BadHex(String),
    EmptyName,
    DuplicateTheme(String),
    NoSuchTheme(String),
    IncompletePalette(String),
}

impl fmt::Display for ThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadHex(h) => write!(f, "bad hex color {h}"),
            Self::EmptyName => write!(f, "theme name must not be empty"),
            Self::DuplicateTheme(n) => write!(f, "duplicate theme {n}"),
            Self::NoSuchTheme(n) => write!(f, "no such theme {n}"),
            Self::IncompletePalette(n) => write!(f, "theme {n} missing palette keys"),
        }
    }
}
impl std::error::Error for ThemeError {}

/// Built-in presets (SigmaOS/Wallust defaults).
pub fn presets() -> Vec<Theme> {
    vec![
        Theme {
            name: "catppuccin-mocha".into(), variant: Variant::Dark,
            palette: [
                ("background", "#1e1e2e"), ("foreground", "#cdd6f4"), ("cursor", "#f5e0dc"),
                ("selection", "#585b70"), ("primary", "#89b4fa"), ("secondary", "#cba6f7"),
                ("accent", "#f5c2e7"), ("error", "#f38ba8"), ("warning", "#fab387"),
                ("success", "#a6e3a1"), ("info", "#89dceb"),
                ("surface0", "#313244"), ("surface1", "#45475a"), ("surface2", "#585b70"),
                ("overlay0", "#6c7086"),
            ].iter().map(|(k,v)| (k.to_string(), Color::from_hex(v).unwrap())).collect(),
        },
        Theme {
            name: "tokyo-night".into(), variant: Variant::Dark,
            palette: [
                ("background", "#1a1b26"), ("foreground", "#c0caf5"), ("cursor", "#c0caf5"),
                ("selection", "#283457"), ("primary", "#7aa2f7"), ("secondary", "#bb9af7"),
                ("accent", "#7dcfff"), ("error", "#f7768e"), ("warning", "#e0af68"),
                ("success", "#9ece6a"), ("info", "#7dcfff"),
                ("surface0", "#24283b"), ("surface1", "#414868"), ("surface2", "#4f5779"),
                ("overlay0", "#565f89"),
            ].iter().map(|(k,v)| (k.to_string(), Color::from_hex(v).unwrap())).collect(),
        },
        Theme {
            name: "nord".into(), variant: Variant::Dark,
            palette: [
                ("background", "#2e3440"), ("foreground", "#d8dee9"), ("cursor", "#d8dee9"),
                ("selection", "#434c5e"), ("primary", "#88c0d0"), ("secondary", "#81a1c1"),
                ("accent", "#8fbcbb"), ("error", "#bf616a"), ("warning", "#d08770"),
                ("success", "#a3be8c"), ("info", "#5e81ac"),
                ("surface0", "#3b4252"), ("surface1", "#434c5e"), ("surface2", "#4c566a"),
                ("overlay0", "#4c566a"),
            ].iter().map(|(k,v)| (k.to_string(), Color::from_hex(v).unwrap())).collect(),
        },
        Theme {
            name: "gruvbox-dark".into(), variant: Variant::Dark,
            palette: [
                ("background", "#282828"), ("foreground", "#ebdbb2"), ("cursor", "#ebdbb2"),
                ("selection", "#504945"), ("primary", "#83a598"), ("secondary", "#8ec07c"),
                ("accent", "#d3869b"), ("error", "#fb4934"), ("warning", "#fabd2f"),
                ("success", "#b8bb26"), ("info", "#fe8019"),
                ("surface0", "#3c3836"), ("surface1", "#504945"), ("surface2", "#665c54"),
                ("overlay0", "#7c6f64"),
            ].iter().map(|(k,v)| (k.to_string(), Color::from_hex(v).unwrap())).collect(),
        },
        Theme {
            name: "catppuccin-latte".into(), variant: Variant::Light,
            palette: [
                ("background", "#eff1f5"), ("foreground", "#4c4f69"), ("cursor", "#dc8a78"),
                ("selection", "#acb0be"), ("primary", "#1e66f5"), ("secondary", "#8839ef"),
                ("accent", "#ea76cb"), ("error", "#d20f39"), ("warning", "#fe640b"),
                ("success", "#40a02b"), ("info", "#04a5e5"),
                ("surface0", "#ccd0da"), ("surface1", "#bcc0cc"), ("surface2", "#acb0be"),
                ("overlay0", "#9ca0b0"),
            ].iter().map(|(k,v)| (k.to_string(), Color::from_hex(v).unwrap())).collect(),
        },
    ]
}

pub struct SigmaOSTheme {
    themes: BTreeMap<String, Theme>,
    current: Option<String>,
}

impl SigmaOSTheme {
    pub fn new() -> Self {
        let mut themes = BTreeMap::new();
        for t in presets() { themes.insert(t.name.clone(), t); }
        Self { themes, current: None }
    }

    pub fn register(&mut self, theme: Theme) -> Result<(), ThemeError> {
        if theme.name.trim().is_empty() { return Err(ThemeError::EmptyName); }
        if !theme.is_complete() { return Err(ThemeError::IncompletePalette(theme.name.clone())); }
        if self.themes.contains_key(&theme.name) { return Err(ThemeError::DuplicateTheme(theme.name)); }
        self.themes.insert(theme.name.clone(), theme);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Theme> { self.themes.get(name) }
    pub fn themes(&self) -> impl Iterator<Item = &Theme> { self.themes.values() }
    pub fn current(&self) -> Option<&Theme> { self.current.as_ref().and_then(|n| self.themes.get(n)) }

    /// Apply a theme by name; returns the rendered CSS for callers to write.
    pub fn apply(&mut self, name: &str) -> Result<String, ThemeError> {
        let t = self.themes.get(name).ok_or_else(|| ThemeError::NoSuchTheme(name.into()))?;
        self.current = Some(name.into());
        Ok(t.css_vars())
    }

    /// Cycle to next theme (sorted order); returns new current name.
    pub fn cycle(&mut self) -> Option<String> {
        if self.themes.is_empty() { return None; }
        let names: Vec<&String> = self.themes.keys().collect();
        let next = match &self.current {
            None => names[0].clone(),
            Some(cur) => {
                let i = names.iter().position(|n| *n == cur).unwrap_or(0);
                names[(i + 1) % names.len()].clone()
            }
        };
        self.current = Some(next.clone());
        Some(next)
    }

    /// Persist the current theme name to a state file.
    pub fn save_state(&self, path: &std::path::Path) -> Result<(), String> {
        if let Some(p) = path.parent() { std::fs::create_dir_all(p).map_err(|e| e.to_string())?; }
        match &self.current {
            Some(n) => std::fs::write(path, format!("{n}\n")).map_err(|e| e.to_string()),
            None => std::fs::write(path, "").map_err(|e| e.to_string()),
        }
    }

    /// Restore current theme from a state file (no-op if missing/unknown).
    pub fn load_state(&mut self, path: &std::path::Path) {
        if let Ok(txt) = std::fs::read_to_string(path) {
            let name = txt.trim();
            if self.themes.contains_key(name) { self.current = Some(name.to_string()); }
        }
    }

    /// Default state path.
    pub fn state_path() -> std::path::PathBuf {
        if let Ok(p) = std::env::var("SIGMAOS_THEME_FILE") { return std::path::PathBuf::from(p); }
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
        std::path::PathBuf::from(home).join(".config/sigmaos/theme")
    }

    pub fn status_line(&self) -> String {
        match self.current() {
            Some(t) => format!("theme={} variant={} colors={}", t.name, t.variant.label(), t.palette.len()),
            None => "theme=<none>".into(),
        }
    }
}

impl Default for SigmaOSTheme {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_parse_and_hex() {
        let c = Color::from_hex("#89b4fa").unwrap();
        assert_eq!(c.hex(), "#89b4fa");
        assert_eq!(Color::from_hex("ff0000").unwrap().r, 255);
        assert!(Color::from_hex("xyz").is_err());
        assert!(Color::from_hex("#12345").is_err());
    }

    #[test]
    fn luminance_and_ansi() {
        assert!(Color::rgb(255,255,255).is_light());
        assert!(!Color::rgb(0,0,0).is_light());
        assert!(Color::rgb(255,0,0).ansi256() <= 231);
    }

    #[test]
    fn presets_complete() {
        let p = presets();
        assert!(p.len() >= 4);
        for t in &p { assert!(t.is_complete(), "{} incomplete", t.name); }
    }

    #[test]
    fn registry_apply_cycle() {
        let mut m = SigmaOSTheme::new();
        assert!(m.apply("catppuccin-mocha").is_ok());
        assert_eq!(m.current().unwrap().name, "catppuccin-mocha");
        assert!(m.status_line().contains("catppuccin-mocha"));
        let next = m.cycle().unwrap();
        assert_ne!(next, "catppuccin-mocha");
        assert!(m.apply("nope").is_err());
    }

    #[test]
    fn css_and_hyprland_render() {
        let m = SigmaOSTheme::new();
        let t = m.get("nord").unwrap();
        let css = t.css_vars();
        assert!(css.contains("--background: #2e3440;"));
        let hyp = t.hyprland_colors();
        assert!(hyp.contains("col.background = #2e3440"));
        assert_eq!(t.terminal_colors().len(), 16);
    }

    #[test]
    fn register_validation() {
        let mut m = SigmaOSTheme::new();
        assert!(m.register(Theme { name: " ".into(), variant: Variant::Dark, palette: BTreeMap::new() }).is_err());
        // duplicate of built-in
        let dup = presets()[0].clone();
        assert!(m.register(dup).is_err());
        // incomplete palette
        let inc = Theme { name: "bad".into(), variant: Variant::Dark, palette: BTreeMap::new() };
        assert!(m.register(inc).is_err());
    }

    #[test]
    fn custom_theme_register() {
        let mut m = SigmaOSTheme::new();
        let mut pal = BTreeMap::new();
        for k in PALETTE_KEYS { pal.insert(k.to_string(), Color::rgb(10,20,30)); }
        let t = Theme { name: "custom".into(), variant: Variant::Dark, palette: pal };
        m.register(t).unwrap();
        assert!(m.get("custom").is_some());
        assert_eq!(m.themes().count(), presets().len() + 1);
    }
}
