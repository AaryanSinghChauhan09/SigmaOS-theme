# SigmaOS Theme Engine (`SigmaOS-theme`)

Visual theme engine, dynamic color scheme extractor, Wallust manager, and community theme installer for SigmaOS and Omarchy Linux.

---

## Capabilities & Tooling

1. **System-Wide Theming (`bin/sigmaos-theme`)**:
   - 8 hand-crafted curated themes:
     `catppuccin-mocha`, `tokyo-night`, `gruvbox-dark`, `nord`, `everforest`, `matte-black`, `cyber`, `zenith`
   - Real-time reloading of terminals (Foot, Kitty, Ghostty), Hyprland window borders, Waybar, and Quickshell.

2. **Community Themes (`bin/sigmaos-theme-install`)**:
   - Search, download, preview, and apply popular community themes:
     `decay-green`, `dracula`, `solarized-dark`, `ayu-dark`, `rose-pine`, `kanagawa`, `oxocarbon`, `tokyonight-storm`, `nightfox`, `catppuccin-latte`.

3. **Dynamic Wallpaper Generation (`bin/sigmaos-wallpaper-set`)**:
   - Extracts 16-color palette from any image wallpaper via Wallust.
   - Synchronizes multi-monitor wallpapers with Hyprpaper, Swaybg, Feh, or Nitrogen.

4. **Rust Core Engine (`src/theme_engine.rs`)**:
   - Strong type-safe theme registry and palette parser.

---

## Commands

```bash
sigmaos-theme list                # List bundled themes
sigmaos-theme catppuccin-mocha    # Apply theme
sigmaos-theme next                # Cycle next theme
sigmaos-theme-install list-community # Explore community themes
sigmaos-theme-install install dracula # Install community theme
sigmaos-wallpaper-set <path>      # Extract dynamic theme from wallpaper
```
