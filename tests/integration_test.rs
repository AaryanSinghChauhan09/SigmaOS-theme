use sigmaos_theme::{AccentColor, Config, Theme, ThemeComponent, ThemeRegistry, ThemeVariant, Component};

#[test]
fn test_accent_color_hex_aaa() {
    let color = AccentColor::sigma_blue();
    assert_eq!(color.hex(), "#407BFF");
}

#[test]
fn test_theme_registry_default_aaa() {
    let reg = ThemeRegistry::new();
    assert!(reg.active_theme().is_some());
    assert_eq!(reg.active_theme().unwrap().name, "SigmaOS Default");
}

#[test]
fn test_theme_registry_set_active_aaa() {
    let mut reg = ThemeRegistry::new();
    let dark = Theme {
        name: "SigmaOS Dark".to_string(),
        variant: ThemeVariant::Dark,
        accent: AccentColor::sigma_blue(),
        font_family: "Inter".to_string(),
        icon_theme: "sigma-icons".to_string(),
        cursor_theme: "sigma-cursors".to_string(),
    };
    reg.register(dark);
    let result = reg.set_active("SigmaOS Dark");
    assert!(result.is_ok());
    assert_eq!(reg.active_theme().unwrap().name, "SigmaOS Dark");
}

#[test]
fn test_component_lifecycle_aaa() {
    let mut c = ThemeComponent::new(Config::default());
    assert!(c.initialize().is_ok());
    assert!(c.health_check().is_ok());
    assert!(c.shutdown().is_ok());
}

#[test]
fn test_component_name_aaa() {
    let c = ThemeComponent::new(Config::default());
    assert_eq!(c.name(), "sigmaos-theme");
}
