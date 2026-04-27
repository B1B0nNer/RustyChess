use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Color,
    buffer::Buffer,
};
use artbox::{Renderer, Alignment as ArtAlignment};
use crate::render_board::menu::text_render::text_render;

pub fn render_title(content_layout: &[Rect], buf: &mut Buffer) {
    // Calculate title area width to be tighter
    let title_area = Layout::horizontal([Constraint::Length(110)])
        .flex(ratatui::layout::Flex::Center)
        .split(content_layout[0])[0];

    // Fill background for the entire title area
    for y in 0..title_area.height {
        for x in 0..title_area.width {
            buf[(title_area.x + x, title_area.y + y)].set_bg(Color::Rgb(30, 38, 80));
        }
    }

    let title_grid = Renderer::default()
        .with_alignment(ArtAlignment::Center)
        .render_grid("RUSTY CHESS", title_area.width, 7) // Title height is 7
        .unwrap();

    text_render(title_grid, title_area, Color::Rgb(30, 38, 80), buf);

    let creator_line = ["Created by", "Nikita \"B1B0nNer\" Supereka"];

    for (a, creator) in creator_line.iter().enumerate() {
        let creator_x = title_area.x + (title_area.width.saturating_sub(creator.len() as u16)) / 2;
        for (i, ch) in creator.chars().enumerate() {
            if (creator_x + i as u16) < title_area.x + title_area.width {
                buf[(creator_x + i as u16, title_area.y + 7 + a as u16)].set_char(ch).set_fg(Color::Gray);
            }
        }
    }
}