use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect, Flex},
    style::{Color, Style},
    widgets::{Paragraph, Widget, Block},
};
use ratatui_interact::components::ButtonState;

pub struct ReplayButton<'a> {
    pub state: &'a ButtonState,
}

impl<'a> Widget for ReplayButton<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let replay_layout = Layout::horizontal([Constraint::Length(12)])
            .flex(Flex::Center)
            .split(area);
        
        let button_area = replay_layout[0];
        
        let mut replay_style = Style::default().fg(Color::Black).bg(Color::White);
        if self.state.pressed {
            replay_style = replay_style.bg(Color::Gray);
        } else if self.state.focused {
            replay_style = replay_style.bg(Color::LightCyan);
        }

        let replay_button = Paragraph::new(" REPLAY ")
            .centered()
            .block(Block::bordered().style(replay_style));
        
        replay_button.render(button_area, buf);
    }
}

pub fn get_replay_button_area(area: Rect) -> Rect {
    let replay_layout = Layout::horizontal([Constraint::Length(12)])
        .flex(Flex::Center)
        .split(area);
    replay_layout[0]
}
