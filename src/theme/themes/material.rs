use super::super::Theme;
use ratatui::style::Color;

/// Material Theme inspired color scheme
pub fn material() -> Theme {
    Theme {
        background_left: Color::Rgb(38, 50, 56),
        background_right: Color::Rgb(38, 50, 56),

        editor_line_number: Color::Rgb(84, 110, 122),
        editor_line_number_cursor: Color::Rgb(128, 203, 196),
        editor_separator: Color::Rgb(84, 110, 122),
        editor_cursor_char_bg: Color::Rgb(255, 203, 107),
        editor_cursor_char_fg: Color::Rgb(38, 50, 56),
        editor_cursor_line_bg: Color::Rgb(55, 71, 79),

        file_tree_added: Color::Rgb(195, 232, 141),
        file_tree_deleted: Color::Rgb(255, 83, 112),
        file_tree_modified: Color::Rgb(255, 203, 107),
        file_tree_renamed: Color::Rgb(128, 203, 196),
        file_tree_directory: Color::Rgb(137, 221, 255),
        file_tree_current_file_bg: Color::Rgb(55, 71, 79),
        file_tree_current_file_fg: Color::Rgb(238, 255, 255),
        file_tree_default: Color::Rgb(238, 255, 255),
        file_tree_stats_added: Color::Rgb(195, 232, 141),
        file_tree_stats_deleted: Color::Rgb(255, 83, 112),

        terminal_command: Color::Rgb(238, 255, 255),
        terminal_output: Color::Rgb(84, 110, 122),
        terminal_cursor_bg: Color::Rgb(255, 203, 107),
        terminal_cursor_fg: Color::Rgb(38, 50, 56),

        status_hash: Color::Rgb(255, 203, 107),
        status_author: Color::Rgb(195, 232, 141),
        status_date: Color::Rgb(128, 203, 196),
        status_message: Color::Rgb(238, 255, 255),
        status_no_commit: Color::Rgb(84, 110, 122),

        separator: Color::Rgb(84, 110, 122),

        syntax_keyword: Color::Rgb(199, 146, 234),
        syntax_type: Color::Rgb(255, 203, 107),
        syntax_function: Color::Rgb(130, 170, 255),
        syntax_variable: Color::Rgb(238, 255, 255),
        syntax_string: Color::Rgb(195, 232, 141),
        syntax_number: Color::Rgb(247, 140, 108),
        syntax_comment: Color::Rgb(84, 110, 122),
        syntax_operator: Color::Rgb(137, 221, 255),
        syntax_punctuation: Color::Rgb(144, 164, 174),
        syntax_constant: Color::Rgb(137, 221, 255),
        syntax_parameter: Color::Rgb(255, 203, 107),
        syntax_property: Color::Rgb(128, 203, 196),
        syntax_label: Color::Rgb(199, 146, 234),
    }
}
