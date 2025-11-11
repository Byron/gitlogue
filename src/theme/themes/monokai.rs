use super::super::Theme;
use ratatui::style::Color;

/// Monokai inspired color scheme
pub fn monokai() -> Theme {
    Theme {
        background_left: Color::Rgb(30, 30, 30),
        background_right: Color::Rgb(39, 40, 34),

        editor_line_number: Color::Rgb(117, 113, 94),
        editor_line_number_cursor: Color::Rgb(102, 217, 239),
        editor_separator: Color::Rgb(117, 113, 94),
        editor_cursor_char_bg: Color::Rgb(253, 151, 31),
        editor_cursor_char_fg: Color::Rgb(39, 40, 34),
        editor_cursor_line_bg: Color::Rgb(51, 51, 45),

        file_tree_added: Color::Rgb(166, 226, 46),
        file_tree_deleted: Color::Rgb(249, 38, 114),
        file_tree_modified: Color::Rgb(253, 151, 31),
        file_tree_renamed: Color::Rgb(102, 217, 239),
        file_tree_directory: Color::Rgb(174, 129, 255),
        file_tree_current_file_bg: Color::Rgb(51, 51, 45),
        file_tree_current_file_fg: Color::Rgb(248, 248, 242),
        file_tree_default: Color::Rgb(248, 248, 242),
        file_tree_stats_added: Color::Rgb(166, 226, 46),
        file_tree_stats_deleted: Color::Rgb(249, 38, 114),

        terminal_command: Color::Rgb(248, 248, 242),
        terminal_output: Color::Rgb(117, 113, 94),
        terminal_cursor_bg: Color::Rgb(253, 151, 31),
        terminal_cursor_fg: Color::Rgb(39, 40, 34),

        status_hash: Color::Rgb(230, 219, 116),
        status_author: Color::Rgb(166, 226, 46),
        status_date: Color::Rgb(102, 217, 239),
        status_message: Color::Rgb(248, 248, 242),
        status_no_commit: Color::Rgb(117, 113, 94),

        separator: Color::Rgb(117, 113, 94),

        syntax_keyword: Color::Rgb(249, 38, 114),
        syntax_type: Color::Rgb(102, 217, 239),
        syntax_function: Color::Rgb(166, 226, 46),
        syntax_variable: Color::Rgb(248, 248, 242),
        syntax_string: Color::Rgb(230, 219, 116),
        syntax_number: Color::Rgb(174, 129, 255),
        syntax_comment: Color::Rgb(117, 113, 94),
        syntax_operator: Color::Rgb(249, 38, 114),
        syntax_punctuation: Color::Rgb(248, 248, 242),
        syntax_constant: Color::Rgb(174, 129, 255),
        syntax_parameter: Color::Rgb(253, 151, 31),
        syntax_property: Color::Rgb(166, 226, 46),
        syntax_label: Color::Rgb(249, 38, 114),
    }
}
