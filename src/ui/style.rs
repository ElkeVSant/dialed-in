use ratatui::style::Color;

pub const AROMA: ScoreColorScale = ScoreColorScale {
    lightest: Color::Rgb(212, 219, 252),
    light: Color::Rgb(200, 202, 255),
    medium: Color::Rgb(190, 180, 252),
    dark: Color::Rgb(172, 163, 228),
    darkest: Color::Rgb(151, 143, 203),
};

pub const SWEETNESS: ScoreColorScale = ScoreColorScale {
    lightest: Color::Rgb(249, 249, 250),
    light: Color::Rgb(249, 235, 238),
    medium: Color::Rgb(245, 223, 229),
    dark: Color::Rgb(240, 209, 219),
    darkest: Color::Rgb(234, 195, 209),
};

pub const ACIDITY: ScoreColorScale = ScoreColorScale {
    lightest: Color::Rgb(246, 248, 236),
    light: Color::Rgb(230, 239, 214),
    medium: Color::Rgb(200, 219, 168),
    dark: Color::Rgb(181, 209, 145),
    darkest: Color::Rgb(163, 199, 125),
};

pub const BODY: ScoreColorScale = ScoreColorScale {
    lightest: Color::Rgb(244, 249, 249),
    light: Color::Rgb(225, 240, 241),
    medium: Color::Rgb(210, 232, 233),
    dark: Color::Rgb(189, 221, 225),
    darkest: Color::Rgb(168, 211, 217),
};

pub const AFTERTASTE: ScoreColorScale = ScoreColorScale {
    lightest: Color::Rgb(254, 247, 242),
    light: Color::Rgb(251, 236, 224),
    medium: Color::Rgb(247, 224, 208),
    dark: Color::Rgb(242, 211, 193),
    darkest: Color::Rgb(238, 200, 179),
};

pub struct ScoreColorScale {
    lightest: Color,
    light: Color,
    medium: Color,
    dark: Color,
    darkest: Color,
}

impl ScoreColorScale {
    pub fn for_score(&self, score: u8) -> Color {
        match score {
            1 => self.lightest,
            2 => self.light,
            3 => self.medium,
            4 => self.dark,
            5 => self.darkest,
            _ => Color::Reset,
        }
    }
}
