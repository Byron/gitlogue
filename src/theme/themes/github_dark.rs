use super::super::Theme;
use ratatui::style::Color;

/// GitHub Dark inspired color scheme
pub fn github_dark() -> Theme {
    Theme {
        background_left: Color::Rgb(13, 17, 23),
        background_right: Color::Rgb(22, 27, 34),

        editor_line_number: Color::Rgb(110, 118, 129),
        editor_line_number_cursor: Color::Rgb(88, 166, 255),
        editor_separator: Color::Rgb(48, 54, 61),
        editor_cursor_char_bg: Color::Rgb(88, 166, 255),
        editor_cursor_char_fg: Color::Rgb(22, 27, 34),
        editor_cursor_line_bg: Color::Rgb(33, 38, 45),

        file_tree_added: Color::Rgb(63, 185, 80),
        file_tree_deleted: Color::Rgb(248, 81, 73),
        file_tree_modified: Color::Rgb(219, 109, 40),
        file_tree_renamed: Color::Rgb(88, 166, 255),
        file_tree_directory: Color::Rgb(88, 166, 255),
        file_tree_current_file_bg: Color::Rgb(33, 38, 45),
        file_tree_current_file_fg: Color::Rgb(230, 237, 243),
        file_tree_default: Color::Rgb(201, 209, 217),
        file_tree_stats_added: Color::Rgb(63, 185, 80),
        file_tree_stats_deleted: Color::Rgb(248, 81, 73),

        terminal_command: Color::Rgb(230, 237, 243),
        terminal_output: Color::Rgb(110, 118, 129),
        terminal_cursor_bg: Color::Rgb(88, 166, 255),
        terminal_cursor_fg: Color::Rgb(22, 27, 34),

        status_hash: Color::Rgb(219, 171, 9),
        status_author: Color::Rgb(63, 185, 80),
        status_date: Color::Rgb(88, 166, 255),
        status_message: Color::Rgb(230, 237, 243),
        status_no_commit: Color::Rgb(110, 118, 129),

        separator: Color::Rgb(48, 54, 61),

        syntax_keyword: Color::Rgb(255, 123, 114),
        syntax_type: Color::Rgb(255, 186, 77),
        syntax_function: Color::Rgb(210, 153, 255),
        syntax_variable: Color::Rgb(201, 209, 217),
        syntax_string: Color::Rgb(168, 219, 181),
        syntax_number: Color::Rgb(121, 192, 255),
        syntax_comment: Color::Rgb(139, 148, 158),
        syntax_operator: Color::Rgb(255, 123, 114),
        syntax_punctuation: Color::Rgb(201, 209, 217),
        syntax_constant: Color::Rgb(121, 192, 255),
        syntax_parameter: Color::Rgb(255, 186, 77),
        syntax_property: Color::Rgb(121, 192, 255),
        syntax_label: Color::Rgb(210, 153, 255),
    }
}
