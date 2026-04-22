use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Color,
    buffer::Buffer,
};
use artbox::{Renderer, Alignment as ArtAlignment};

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

    // Vertically center title
    let v_offset = 1;

    for (y, row) in title_grid.chars.iter().enumerate() {
        let h_offset = (title_area.width.saturating_sub(row.len() as u16)) / 2;
        for (x, cell) in row.iter().enumerate() {
            if cell.ch != ' ' {
                let cell_buf = &mut buf[(title_area.x + h_offset + x as u16, title_area.y + v_offset + y as u16)];
                cell_buf.set_char(cell.ch).set_fg(Color::White);
            }
        }
    }

    // Creator credentials
    let creator_line1 = "Created by";
    let creator_line2 = "Nikita \"B1B0nNer\" Supereka";

    let creator_y1 = v_offset + 6;
    let creator_y2 = v_offset + 7;

    let creator_x1 = title_area.x + (title_area.width.saturating_sub(creator_line1.len() as u16)) / 2;
    let creator_x2 = title_area.x + (title_area.width.saturating_sub(creator_line2.len() as u16)) / 2;

    for (i, ch) in creator_line1.chars().enumerate() {
        if (creator_x1 + i as u16) < title_area.x + title_area.width {
            buf[(creator_x1 + i as u16, title_area.y + creator_y1)].set_char(ch).set_fg(Color::Gray);
        }
    }
    for (i, ch) in creator_line2.chars().enumerate() {
        if (creator_x2 + i as u16) < title_area.x + title_area.width {
            buf[(creator_x2 + i as u16, title_area.y + creator_y2)].set_char(ch).set_fg(Color::Gray);
        }
    }
}