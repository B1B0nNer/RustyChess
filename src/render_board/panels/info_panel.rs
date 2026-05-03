use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Paragraph, Widget},
    buffer::Buffer,
};

pub struct InfoPanel<'a> {
    pub turn: char,
    pub history: &'a Vec<String>,
    pub white_time: u32,
    pub black_time: u32,
    pub is_unlimited: bool,
}

impl<'a> Widget for InfoPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let format_time = |secs: u32| -> String {
            if self.is_unlimited { "∞".to_string() }
            else { format!("{:02}:{:02}", secs / 60, secs % 60) }
        };

        let chunks = Layout::vertical([
            Constraint::Length(3), // Black Timer
            Constraint::Min(0),    // History Block
            Constraint::Length(3), // White Timer
        ])
            .split(area);

        // Render Black Timer
        let black_out = self.black_time == 0 && !self.is_unlimited;
        let black_style = if black_out { Style::default().fg(Color::Red) }
        else if self.turn == 'b' { Style::default().fg(Color::Yellow) }
        else { Style::default().fg(Color::Rgb(103, 115, 122)) };

        Paragraph::new(format_time(self.black_time))
            .block(Block::bordered().title(" BLACK ").border_style(black_style))
            .centered()
            .style(black_style.add_modifier(Modifier::BOLD))
            .render(chunks[0], buf);

        // Render History Block
        let info_block = Block::bordered()
            .title(" HISTORY ")
            .border_type(BorderType::Double)
            .style(Style::default().fg(Color::Rgb(103, 115, 122)).bg(Color::Rgb(30, 38, 80)));

        let history_inner = info_block.inner(chunks[1]);
        info_block.render(chunks[1], buf);

        // Render History List
        let history_text = if self.history.is_empty() {
            "No moves yet...".to_string()
        } else {
            let max_moves = history_inner.height as usize;
            let start = self.history.len().saturating_sub(max_moves);
            self.history[start..].join("\n")
        };

        Paragraph::new(history_text)
            .style(Style::default().fg(Color::Gray))
            .render(history_inner, buf);

        // Render White Timer
        let white_out = self.white_time == 0 && !self.is_unlimited;
        let white_style = if white_out { Style::default().fg(Color::Red) }
        else if self.turn == 'w' { Style::default().fg(Color::Yellow) }
        else { Style::default().fg(Color::Rgb(103, 115, 122)) };

        Paragraph::new(format_time(self.white_time))
            .block(Block::bordered().title(" WHITE ").border_style(white_style))
            .centered()
            .style(white_style.add_modifier(Modifier::BOLD))
            .render(chunks[2], buf);
    }
}