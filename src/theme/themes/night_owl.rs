use super::super::Theme;
use ratatui::style::Color;

/// Night Owl inspired color scheme
pub fn night_owl() -> Theme {
    Theme {
        background_left: Color::Rgb(1, 22, 39),
        background_right: Color::Rgb(1, 22, 39),

        editor_line_number: Color::Rgb(78, 121, 147),
        editor_line_number_cursor: Color::Rgb(122, 162, 247),
        editor_separator: Color::Rgb(1, 76, 134),
        editor_cursor_char_bg: Color::Rgb(122, 162, 247),
        editor_cursor_char_fg: Color::Rgb(1, 22, 39),
        editor_cursor_line_bg: Color::Rgb(1, 41, 72),

        file_tree_added: Color::Rgb(173, 219, 103),
        file_tree_deleted: Color::Rgb(239, 83, 80),
        file_tree_modified: Color::Rgb(255, 213, 128),
        file_tree_renamed: Color::Rgb(122, 162, 247),
        file_tree_directory: Color::Rgb(130, 170, 255),
        file_tree_current_file_bg: Color::Rgb(1, 41, 72),
        file_tree_current_file_fg: Color::Rgb(214, 222, 235),
        file_tree_default: Color::Rgb(214, 222, 235),
        file_tree_stats_added: Color::Rgb(173, 219, 103),
        file_tree_stats_deleted: Color::Rgb(239, 83, 80),

        terminal_command: Color::Rgb(214, 222, 235),
        terminal_output: Color::Rgb(78, 121, 147),
        terminal_cursor_bg: Color::Rgb(122, 162, 247),
        terminal_cursor_fg: Color::Rgb(1, 22, 39),

        status_hash: Color::Rgb(255, 203, 107),
        status_author: Color::Rgb(173, 219, 103),
        status_date: Color::Rgb(122, 162, 247),
        status_message: Color::Rgb(214, 222, 235),
        status_no_commit: Color::Rgb(78, 121, 147),

        separator: Color::Rgb(1, 76, 134),

        syntax_keyword: Color::Rgb(199, 146, 234),
        syntax_type: Color::Rgb(255, 203, 107),
        syntax_function: Color::Rgb(130, 170, 255),
        syntax_variable: Color::Rgb(214, 222, 235),
        syntax_string: Color::Rgb(173, 219, 103),
        syntax_number: Color::Rgb(247, 140, 108),
        syntax_comment: Color::Rgb(78, 121, 147),
        syntax_operator: Color::Rgb(199, 146, 234),
        syntax_punctuation: Color::Rgb(127, 132, 142),
        syntax_constant: Color::Rgb(128, 203, 196),
        syntax_parameter: Color::Rgb(255, 203, 107),
        syntax_property: Color::Rgb(122, 162, 247),
        syntax_label: Color::Rgb(255, 88, 116),
    }
}
