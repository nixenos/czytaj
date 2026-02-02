use iced::Theme;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppTheme {
    CatppuccinMocha,
    CatppuccinMacchiato,
    CatppuccinFrappe,
    CatppuccinLatte,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
}

impl std::fmt::Display for AppTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AppTheme {
    pub fn all() -> Vec<AppTheme> {
        vec![
            AppTheme::CatppuccinMocha,
            AppTheme::CatppuccinMacchiato,
            AppTheme::CatppuccinFrappe,
            AppTheme::CatppuccinLatte,
            AppTheme::TokyoNight,
            AppTheme::TokyoNightStorm,
            AppTheme::TokyoNightLight,
            AppTheme::Dracula,
            AppTheme::Nord,
            AppTheme::SolarizedLight,
            AppTheme::SolarizedDark,
            AppTheme::GruvboxLight,
            AppTheme::GruvboxDark,
            AppTheme::KanagawaWave,
            AppTheme::KanagawaDragon,
            AppTheme::KanagawaLotus,
            AppTheme::Moonfly,
            AppTheme::Nightfly,
            AppTheme::Oxocarbon,
        ]
    }

    pub fn to_iced_theme(self) -> Theme {
        match self {
            AppTheme::CatppuccinMocha => Theme::CatppuccinMocha,
            AppTheme::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            AppTheme::CatppuccinFrappe => Theme::CatppuccinFrappe,
            AppTheme::CatppuccinLatte => Theme::CatppuccinLatte,
            AppTheme::TokyoNight => Theme::TokyoNight,
            AppTheme::TokyoNightStorm => Theme::TokyoNightStorm,
            AppTheme::TokyoNightLight => Theme::TokyoNightLight,
            AppTheme::Dracula => Theme::Dracula,
            AppTheme::Nord => Theme::Nord,
            AppTheme::SolarizedLight => Theme::SolarizedLight,
            AppTheme::SolarizedDark => Theme::SolarizedDark,
            AppTheme::GruvboxLight => Theme::GruvboxLight,
            AppTheme::GruvboxDark => Theme::GruvboxDark,
            AppTheme::KanagawaWave => Theme::KanagawaWave,
            AppTheme::KanagawaDragon => Theme::KanagawaDragon,
            AppTheme::KanagawaLotus => Theme::KanagawaLotus,
            AppTheme::Moonfly => Theme::Moonfly,
            AppTheme::Nightfly => Theme::Nightfly,
            AppTheme::Oxocarbon => Theme::Oxocarbon,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            AppTheme::CatppuccinMocha => "Catppuccin Mocha",
            AppTheme::CatppuccinMacchiato => "Catppuccin Macchiato",
            AppTheme::CatppuccinFrappe => "Catppuccin Frappé",
            AppTheme::CatppuccinLatte => "Catppuccin Latte",
            AppTheme::TokyoNight => "Tokyo Night",
            AppTheme::TokyoNightStorm => "Tokyo Night Storm",
            AppTheme::TokyoNightLight => "Tokyo Night Light",
            AppTheme::Dracula => "Dracula",
            AppTheme::Nord => "Nord",
            AppTheme::SolarizedLight => "Solarized Light",
            AppTheme::SolarizedDark => "Solarized Dark",
            AppTheme::GruvboxLight => "Gruvbox Light",
            AppTheme::GruvboxDark => "Gruvbox Dark",
            AppTheme::KanagawaWave => "Kanagawa Wave",
            AppTheme::KanagawaDragon => "Kanagawa Dragon",
            AppTheme::KanagawaLotus => "Kanagawa Lotus",
            AppTheme::Moonfly => "Moonfly",
            AppTheme::Nightfly => "Nightfly",
            AppTheme::Oxocarbon => "Oxocarbon",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub theme: AppTheme,
    pub show_images: bool,
    pub show_excerpts: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: AppTheme::CatppuccinMocha,
            show_images: true,
            show_excerpts: true,
        }
    }
}
