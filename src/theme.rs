use termimad::crossterm::style::Color::*;

use termimad::*;

pub fn default() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.bold.set_fg(Yellow);
    skin.italic.set_bg(Rgb {
        r: 252,
        g: 255,
        b: 231,
    });
    skin.bullet = StyledChar::from_fg_char(Yellow, '⟡');
    skin.set_headers_fg(Rgb {
        r: 235,
        g: 69,
        b: 95,
    });
    skin.quote_mark = StyledChar::from_fg_char(Yellow, '▐');
    skin.quote_mark.set_fg(Rgb {
        r: 186,
        g: 215,
        b: 233,
    });
    skin.inline_code.set_fg(Rgb {
        r: 186,
        g: 215,
        b: 233,
    });
    skin.italic.set_fg(Rgb {
        r: 43,
        g: 52,
        b: 103,
    });

    skin
}
