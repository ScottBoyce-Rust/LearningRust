use cli_utils::colors::{Color, ColorString};

#[test]
fn test_red_color() {
    let mut color_string = ColorString {
        color: Color::Red,
        string: "Red".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[31mRed\x1b[0m")
}
