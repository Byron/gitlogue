use crate::animation::AnimationEngine;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct TerminalPane;

impl TerminalPane {
    pub fn render(&self, f: &mut Frame, area: Rect, engine: &AnimationEngine) {
        let block = Block::default()
            .title("Terminal")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));

        // Get visible lines based on area height
        let content_height = area.height.saturating_sub(2) as usize; // Subtract borders
        let total_lines = engine.terminal_lines.len();

        let lines: Vec<Line> = if total_lines > 0 {
            let start_idx = total_lines.saturating_sub(content_height);
            engine.terminal_lines[start_idx..]
                .iter()
                .map(|line| {
                    if line.starts_with("$ ") {
                        // Command line - show in green with bold
                        Line::from(Span::styled(
                            line.clone(),
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        ))
                    } else {
                        // Output line - normal style
                        Line::from(line.clone())
                    }
                })
                .collect()
        } else {
            vec![Line::from("")]
        };

        let content = Paragraph::new(lines).block(block);
        f.render_widget(content, area);
    }
}
