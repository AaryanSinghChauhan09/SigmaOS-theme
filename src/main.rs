//! SigmaOS-theme — Wallust-style theme CLI.
//!
//! Usage:
//!   sigmaos-theme list
//!   sigmaos-theme current
//!   sigmaos-theme apply <name>
//!   sigmaos-theme cycle
//!   sigmaos-theme css <name>      (print CSS vars)
//!   sigmaos-theme hyprland <name> (print hyprland col.* lines)
//!   sigmaos-theme colors <name>   (print 16 ANSI color indices)

use theme::{SigmaOSTheme, ThemeError};
use std::process::exit;

fn usage() -> ! {
    eprintln!("usage: sigmaos-theme <list|current|apply|cycle|css|hyprland|colors> [name]");
    exit(2);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() { usage(); }
    let state = SigmaOSTheme::state_path();
    let mut themes = SigmaOSTheme::new();
    themes.load_state(&state);
    let cmd = args[0].as_str();
    let name = args.get(1).map(|s| s.as_str());

    let result: Result<(), String> = match cmd {
        "list" => {
            for t in themes.themes() {
                println!("{:<20} {:<5} {} colors", t.name, t.variant.label(), t.palette.len());
            }
            Ok(())
        }
        "current" => { println!("{}", themes.status_line()); Ok(()) }
        "apply" => {
            let n = name.unwrap_or_else(|| usage());
            themes.apply(n).map(|_| println!("applied {n}")).map_err(|e| e.to_string())
        }
        "cycle" => {
            match themes.cycle() {
                Some(n) => { println!("cycled -> {n}"); Ok(()) }
                None => Err("no themes".into()),
            }
        }
        "css" => {
            let n = name.unwrap_or_else(|| usage());
            match themes.get(n) {
                Some(t) => { print!("{}", t.css_vars()); Ok(()) }
                None => Err(ThemeError::NoSuchTheme(n.into()).to_string()),
            }
        }
        "hyprland" => {
            let n = name.unwrap_or_else(|| usage());
            match themes.get(n) {
                Some(t) => { print!("{}", t.hyprland_colors()); Ok(()) }
                None => Err(ThemeError::NoSuchTheme(n.into()).to_string()),
            }
        }
        "colors" => {
            let n = name.unwrap_or_else(|| usage());
            match themes.get(n) {
                Some(t) => {
                    let idx = t.terminal_colors();
                    println!("{}", idx.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "));
                    Ok(())
                }
                None => Err(ThemeError::NoSuchTheme(n.into()).to_string()),
            }
        }
        _ => usage(),
    };

    match result {
        Ok(()) => {
            // persist current theme after mutating commands
            if matches!(cmd, "apply" | "cycle") {
                if let Err(e) = themes.save_state(&state) { eprintln!("sigmaos-theme: warn: save state: {e}"); }
            }
        }
        Err(e) => { eprintln!("sigmaos-theme: {e}"); exit(1); }
    }
}
