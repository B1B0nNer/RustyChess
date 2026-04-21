use ratatui::{
    layout::{Constraint, Layout, Flex, Rect, Margin},
    style::{Color, Style},
    widgets::{Block, Paragraph, Widget, Clear},
    buffer::Buffer,
};
use crate::render_board::render_board::get_ascii_art;
use crate::Game;

pub struct PromotionPanel<'a> {
    pub game: &'a Game,
}

impl<'a> Widget for PromotionPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(_) = self.game.promotion {
            let dead_figures = if self.game.turn == 'w' {
                &self.game.captured_by_black
            } else {
                &self.game.captured_by_white
            };

            let overlay_area = Layout::vertical([Constraint::Length(9)])
                .flex(Flex::Center)
                .split(area);
            
            let overlay_box = Layout::horizontal([Constraint::Length(70)])
                .flex(Flex::Center)
                .split(overlay_area[0])[0];

            Clear.render(overlay_box, buf);
            Block::bordered()
                .title(" SELECT PROMOTION ")
                .style(Style::default().bg(Color::Blue).fg(Color::White))
                .render(overlay_box, buf);

            if dead_figures.is_empty() {
                let msg = Paragraph::new("No captured pieces to promote to!\n(Wait, this shouldn't happen in this mode?)").centered();
                msg.render(overlay_box.inner(Margin { horizontal: 1, vertical: 2 }), buf);
            } else {
                let item_width = 10;
                let items_count = dead_figures.len();
                
                let items_layout = Layout::horizontal(vec![Constraint::Length(item_width); items_count])
                    .flex(Flex::Center)
                    .split(overlay_box.inner(Margin { horizontal: 1, vertical: 1 }));

                for (idx, piece_code) in dead_figures.iter().enumerate() {
                    let item_area = items_layout[idx];
                    
                    let fg_color = if piece_code.starts_with('w') {
                        Color::Rgb(242, 242, 209)
                    } else {
                        Color::Black
                    };

                    let piece_str = get_ascii_art(piece_code);
                    let piece_height = piece_str.lines().count() as u16;
                    
                    Block::bordered()
                        .style(Style::default().fg(Color::White))
                        .render(item_area, buf);

                    if !piece_str.is_empty() {
                        let piece_layout = Layout::vertical([
                            Constraint::Length(1), // Top border
                            Constraint::Fill(1),   // Flexible space
                            Constraint::Length(piece_height), // The art
                            Constraint::Fill(1),   // Flexible space
                            Constraint::Length(1), // Bottom border
                        ]).split(item_area);

                        Paragraph::new(piece_str)
                            .centered()
                            .style(Style::default().fg(fg_color))
                            .render(piece_layout[2], buf);
                    }
                }
            }
        }
    }
}

pub fn get_promotion_areas(area: Rect, game: &Game) -> Vec<(Rect, usize)> {
    let mut areas = Vec::new();
    if game.promotion.is_none() {
        return areas;
    }

    let dead_figures = if game.turn == 'w' {
        &game.captured_by_black
    } else {
        &game.captured_by_white
    };

    if dead_figures.is_empty() {
        return areas;
    }

    let overlay_area = Layout::vertical([Constraint::Length(9)])
        .flex(Flex::Center)
        .split(area);
    
    let overlay_box = Layout::horizontal([Constraint::Length(70)])
        .flex(Flex::Center)
        .split(overlay_area[0])[0];

    let item_width = 10;
    let items_count = dead_figures.len();
    
    let items_layout = Layout::horizontal(vec![Constraint::Length(item_width); items_count])
        .flex(Flex::Center)
        .split(overlay_box.inner(Margin { horizontal: 1, vertical: 1 }));

    for idx in 0..items_count {
        areas.push((items_layout[idx], idx));
    }

    areas
}
