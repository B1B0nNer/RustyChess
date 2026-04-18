use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Paragraph, Widget},
    buffer::Buffer,
};

pub struct HistoryPanel<'a> {
    pub turn: char,
    pub history: &'a Vec<String>,
    pub is_check: bool,
    pub is_checkmate: bool,
    pub is_stalemate: bool,
}

impl<'a> Widget for HistoryPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let info_block = Block::bordered()
            .title(" HISTORY ")
            .border_type(ratatui::widgets::BorderType::Double)
            .style(Style::default().fg(Color::White).bg(Color::Rgb(211,211,211)));
        
        info_block.render(area, buf);

        let info_layout = Layout::vertical([
            Constraint::Length(5), // Turn info
            Constraint::Min(0),    // History
        ])
        .split(area.inner(ratatui::layout::Margin { horizontal: 1, vertical: 1 }));

        let turn_text = format!("CURRENT TURN:\n  {}", if self.turn == 'w' { "WHITE" } else { "BLACK" });
        let mut turn_style = Style::default().add_modifier(Modifier::BOLD);
        
        let mut status_text = String::new();
        if self.is_checkmate {
            status_text = format!("\n  !!! CHECKMATE !!!\n  {} LOSES", if self.turn == 'w' { "WHITE" } else { "BLACK" });
            turn_style = turn_style.fg(Color::Red);
        } else if self.is_stalemate {
            status_text = "\n  STALEMATE!".to_string();
            turn_style = turn_style.fg(Color::Yellow);
        } else if self.is_check {
            status_text = "\n  !!! CHECK !!!".to_string();
            turn_style = turn_style.fg(Color::LightRed);
        }

        let turn_info = Paragraph::new(format!("{}{}", turn_text, status_text))
            .style(turn_style);
        
        turn_info.render(info_layout[0], buf);

        let history_title = Paragraph::new("HISTORY:").style(Style::default().add_modifier(Modifier::UNDERLINED));
        
        // Show last 15 moves
        let visible_history = if self.history.len() > 15 {
            &self.history[self.history.len()-15..]
        } else {
            &self.history[..]
        };
        let history_text = visible_history.join("\n");
        let history_info = Paragraph::new(format!("\n{}", history_text))
            .style(Style::default().fg(Color::Gray));
        
        let history_area = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
        ]).split(info_layout[1]);

        history_title.render(history_area[0], buf);
        history_info.render(history_area[1], buf);
    }
}
