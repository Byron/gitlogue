use super::super::Theme;
use ratatui::style::Color;

/// Dracula inspired color scheme
pub fn dracula() -> Theme {
    Theme {
        background_left: Color::Rgb(33, 34, 44),
        background_right: Color::Rgb(40, 42, 54),

        editor_line_number: Color::Rgb(98, 114, 164),
        editor_line_number_cursor: Color::Rgb(139, 233, 253),
        editor_separator: Color::Rgb(98, 114, 164),
        editor_cursor_char_bg: Color::Rgb(255, 121, 198),
        editor_cursor_char_fg: Color::Rgb(40, 42, 54),
        editor_cursor_line_bg: Color::Rgb(68, 71, 90),

        file_tree_added: Color::Rgb(80, 250, 123),
        file_tree_deleted: Color::Rgb(255, 85, 85),
        file_tree_modified: Color::Rgb(255, 184, 108),
        file_tree_renamed: Color::Rgb(139, 233, 253),
        file_tree_directory: Color::Rgb(189, 147, 249),
        file_tree_current_file_bg: Color::Rgb(68, 71, 90),
        file_tree_current_file_fg: Color::Rgb(248, 248, 242),
        file_tree_default: Color::Rgb(248, 248, 242),
        file_tree_stats_added: Color::Rgb(80, 250, 123),
        file_tree_stats_deleted: Color::Rgb(255, 85, 85),

        terminal_command: Color::Rgb(248, 248, 242),
        terminal_output: Color::Rgb(98, 114, 164),
        terminal_cursor_bg: Color::Rgb(255, 121, 198),
        terminal_cursor_fg: Color::Rgb(40, 42, 54),

        status_hash: Color::Rgb(241, 250, 140),
        status_author: Color::Rgb(80, 250, 123),
        status_date: Color::Rgb(139, 233, 253),
        status_message: Color::Rgb(248, 248, 242),
        status_no_commit: Color::Rgb(98, 114, 164),

        separator: Color::Rgb(98, 114, 164),

        syntax_keyword: Color::Rgb(255, 121, 198),
        syntax_type: Color::Rgb(139, 233, 253),
        syntax_function: Color::Rgb(80, 250, 123),
        syntax_variable: Color::Rgb(248, 248, 242),
        syntax_string: Color::Rgb(241, 250, 140),
        syntax_number: Color::Rgb(189, 147, 249),
        syntax_comment: Color::Rgb(98, 114, 164),
        syntax_operator: Color::Rgb(255, 121, 198),
        syntax_punctuation: Color::Rgb(248, 248, 242),
        syntax_constant: Color::Rgb(189, 147, 249),
        syntax_parameter: Color::Rgb(255, 184, 108),
        syntax_property: Color::Rgb(80, 250, 123),
        syntax_label: Color::Rgb(255, 121, 198),
    }
}
