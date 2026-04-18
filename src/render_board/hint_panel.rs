use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph, Widget},
    buffer::Buffer,
};

pub struct HintPanel;

impl Widget for HintPanel {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let hint_block = Block::bordered()
            .border_type(BorderType::Plain)
            .style(Style::default().fg(Color::Gray));
        
        let hint = Paragraph::new("Press q to close")
            .centered()
            .block(hint_block);

        hint.render(area, buf);
    }
}
