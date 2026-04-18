use ratatui::{
    layout::{Constraint, Layout, Rect, Margin},
    style::{Color, Style, Modifier},
    widgets::{Block, Paragraph, Widget},
    buffer::Buffer,
};
use std::collections::HashMap;
use crate::render_board::render_board::get_ascii_art;

pub struct CapturedPanel<'a> {
    pub captured_by_white: &'a Vec<&'static str>,
    pub captured_by_black: &'a Vec<&'static str>,
}

impl<'a> Widget for CapturedPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(" CAPTURED ")
            .border_type(ratatui::widgets::BorderType::Double)
            .style(Style::default().fg(Color::White).bg(Color::Rgb(211,211,211)));
        
        block.render(area, buf);

        let inner_area = area.inner(Margin { horizontal: 2, vertical: 2 });
        
        let layout = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(inner_area);

        self.render_captured_section("BY WHITE:", self.captured_by_white, layout[0], buf);
        self.render_captured_section("BY BLACK:", self.captured_by_black, layout[1], buf);
    }
}

impl<'a> CapturedPanel<'a> {
    fn render_captured_section(&self, title: &str, pieces: &[&'static str], area: Rect, buf: &mut Buffer) {
        let mut counts = HashMap::new();
        for &p in pieces {
            *counts.entry(p).or_insert(0) += 1;
        }

        let section_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
        ]).split(area);

        let title_para = Paragraph::new(title)
            .style(Style::default().add_modifier(Modifier::BOLD));
        title_para.render(section_layout[0], buf);

        let mut sorted_pieces: Vec<_> = counts.keys().collect();
        sorted_pieces.sort_by_key(|&&p| {
            let piece_type = if p.len() >= 2 { &p[1..2] } else { p };
            match piece_type {
                "k" => 0,
                "q" => 1,
                "r" => 2,
                "b" => 3,
                "h" => 4,
                "p" => 5,
                _ => 6,
            }
        });

        // Layout pieces in a grid (multiple pieces per row if they fit)
        let mut current_y = section_layout[1].y;
        let mut current_x = section_layout[1].x;
        let section_width = section_layout[1].width;
        let item_width = 10; // Width for one piece art + count
        let item_height = 3;

        for &&p in &sorted_pieces {
            if current_y + item_height > section_layout[1].y + section_layout[1].height {
                break; // No more vertical space
            }

            let art = get_ascii_art(p);
            let fg_color = if p.starts_with('w') {
                Color::Rgb(242, 242, 209)
            } else {
                Color::Black
            };

            let piece_area = Rect::new(current_x, current_y, item_width, item_height);
            let item_layout = Layout::horizontal([
                Constraint::Length(6), // Art width
                Constraint::Min(0),    // Count
            ]).split(piece_area);

            let art_para = Paragraph::new(art)
                .style(Style::default().fg(fg_color).bg(Color::Rgb(211,211,211)))
                .alignment(ratatui::layout::Alignment::Center);
            art_para.render(item_layout[0], buf);

            let count_para = Paragraph::new(format!("x{}", counts[p]))
                .style(Style::default().fg(Color::Black).bg(Color::Rgb(211,211,211)))
                .alignment(ratatui::layout::Alignment::Left);
            
            let count_area = Rect::new(item_layout[1].x, item_layout[1].y + 1, item_layout[1].width, 1);
            count_para.render(count_area, buf);

            // Move to next column
            current_x += item_width + 1;
            
            // If next piece doesn't fit in current row, move to next row
            if current_x + item_width > section_layout[1].x + section_width {
                current_x = section_layout[1].x;
                current_y += item_height + 1; // 3 lines of art + 1 line gap between rows
            }
        }
    }
}
