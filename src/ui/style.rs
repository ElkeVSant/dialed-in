use ratatui::style::Color;

const TOTAL_SCORE: [Color; 15] = [
    Color::Rgb(252, 246, 169),
    Color::Rgb(250, 242, 150),
    Color::Rgb(250, 238, 132),
    Color::Rgb(250, 234, 115),
    Color::Rgb(249, 228, 100),
    Color::Rgb(248, 221, 86),
    Color::Rgb(247, 214, 76),
    Color::Rgb(247, 206, 70),
    Color::Rgb(244, 200, 68),
    Color::Rgb(243, 192, 66),
    Color::Rgb(240, 183, 63),
    Color::Rgb(238, 171, 59),
    Color::Rgb(235, 156, 55),
    Color::Rgb(232, 138, 51),
    Color::Rgb(230, 122, 47),
];

pub(super) const AROMA: ScoreColorScale = ScoreColorScale {
    lightest: Color::Rgb(212, 219, 252),
    light: Color::Rgb(200, 202, 255),
    medium: Color::Rgb(190, 180, 252),
    dark: Color::Rgb(172, 163, 228),
    darkest: Color::Rgb(151, 143, 203),
};

pub(super) const SWEETNESS: ScoreColorScale = ScoreColorScale {
    lightest: Color::Rgb(249, 249, 250),
    light: Color::Rgb(249, 235, 238),
    medium: Color::Rgb(245, 223, 229),
    dark: Color::Rgb(240, 209, 219),
    darkest: Color::Rgb(234, 195, 209),
};

pub(super) const ACIDITY: ScoreColorScale = ScoreColorScale {
    lightest: Color::Rgb(246, 248, 236),
    light: Color::Rgb(230, 239, 214),
    medium: Color::Rgb(200, 219, 168),
    dark: Color::Rgb(181, 209, 145),
    darkest: Color::Rgb(163, 199, 125),
};

pub(super) const BODY: ScoreColorScale = ScoreColorScale {
    lightest: Color::Rgb(244, 249, 249),
    light: Color::Rgb(225, 240, 241),
    medium: Color::Rgb(210, 232, 233),
    dark: Color::Rgb(189, 221, 225),
    darkest: Color::Rgb(168, 211, 217),
};

pub(super) const AFTERTASTE: ScoreColorScale = ScoreColorScale {
    lightest: Color::Rgb(254, 247, 242),
    light: Color::Rgb(251, 236, 224),
    medium: Color::Rgb(247, 224, 208),
    dark: Color::Rgb(242, 211, 193),
    darkest: Color::Rgb(238, 200, 179),
};

pub(super) struct ScoreColorScale {
    lightest: Color,
    light: Color,
    medium: Color,
    dark: Color,
    darkest: Color,
}

impl ScoreColorScale {
    pub(super) fn for_score(&self, score: u8) -> Color {
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

pub(super) fn get_colour_for_total_score(total: u8) -> Color {
    match total {
        0..=5 => Color::White,
        6 | 7 => TOTAL_SCORE[0],
        8 | 9 => TOTAL_SCORE[1],
        10 | 11 => TOTAL_SCORE[2],
        12 | 13 => TOTAL_SCORE[3],
        14 | 15 => TOTAL_SCORE[4],
        16 => TOTAL_SCORE[5],
        17 => TOTAL_SCORE[6],
        18 => TOTAL_SCORE[7],
        19 => TOTAL_SCORE[8],
        20 => TOTAL_SCORE[9],
        21 => TOTAL_SCORE[10],
        22 => TOTAL_SCORE[11],
        23 => TOTAL_SCORE[12],
        24 => TOTAL_SCORE[13],
        25 => TOTAL_SCORE[14],
        _ => Color::Reset,
    }
}
