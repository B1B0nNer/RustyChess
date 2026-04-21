use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect, Alignment},
    style::{Color, Style},
    widgets::{Widget, Paragraph, Block},
};
use ratatui_interact::components::{Button, ButtonState, ButtonStyle};
use crate::render_board::render_board::get_ascii_art;
use crate::render_board::panels::hint_panel::HintPanel;
use artbox::{Renderer, Alignment as ArtAlignment};

pub struct Menu<'a> {
    pub states: &'a [ButtonState; 2],
}

pub fn get_menu_button_areas(area: Rect) -> [Rect; 2] {
    let total_height = 10 + 3 + 17 + 2 + 3; // Title(11) + Sp(3) + Modes(17) + Sp(2) + Hint(3)
    let main_layout = Layout::vertical([
        Constraint::Length(total_height),
    ])
        .flex(ratatui::layout::Flex::Center)
        .split(area);

    let menu_area = main_layout[0];

    let content_layout = Layout::vertical([
        Constraint::Length(10), // Title Area (Title + Creator)
        Constraint::Length(3),  // Spacer
        Constraint::Length(17), // Modes
        Constraint::Length(2),  // Spacer
        Constraint::Length(3),  // Hint Panel
    ])
        .split(menu_area);

    let modes_layout = Layout::horizontal([
        Constraint::Length(60), // Left mode column
        Constraint::Length(0),  // Center spacer
        Constraint::Length(60), // Right mode column
    ])
        .flex(ratatui::layout::Flex::Center)
        .split(content_layout[2]);

    // Normal Mode Area
    let normal_content = Layout::vertical([
        Constraint::Length(10), // Previews
        Constraint::Length(1),  // Spacer
        Constraint::Length(6),  // Button area
    ])
        .flex(ratatui::layout::Flex::Center)
        .split(modes_layout[0]);

    let normal_btn_area = Layout::horizontal([Constraint::Length(50)])
        .flex(ratatui::layout::Flex::Center)
        .split(normal_content[2])[0];

    // Fischer Mode Area
    let fischer_content = Layout::vertical([
        Constraint::Length(10), // Previews
        Constraint::Length(1),  // Spacer
        Constraint::Length(6),  // Button area
    ])
        .flex(ratatui::layout::Flex::Center)
        .split(modes_layout[2]);

    let fischer_btn_area = Layout::horizontal([Constraint::Length(50)])
        .flex(ratatui::layout::Flex::Center)
        .split(fischer_content[2])[0];

    [normal_btn_area, fischer_btn_area]
}

impl<'a> Menu<'a> {
    fn render_preview(&self, area: Rect, buf: &mut Buffer, row: &[&'static str], is_white: bool) {
        let piece_width = 7; // Use 7 specifically to fit 56 in 60
        let total_preview_width = piece_width * 8;
        let cell_height = area.height;

        // Center the 56-width preview in the 60-width area
        let x_offset = (area.width.saturating_sub(total_preview_width)) / 2;

        for i in 0..8 {
            let cell_area = Rect {
                x: area.x + x_offset + (i as u16 * piece_width),
                y: area.y,
                width: piece_width,
                height: cell_height,
            };

            let is_dark = i % 2 != (if is_white { 1 } else { 0 });
            let bg_color = if is_dark {
                Color::Rgb(183, 65, 14) // Rust Orange
            } else {
                Color::Rgb(103, 115, 122) // Rust Silver
            };

            Block::default().style(Style::default().bg(bg_color)).render(cell_area, buf);

            let piece_code = row[i];
            let piece_str = get_ascii_art(piece_code);
            if !piece_str.is_empty() {
                let fg_color = if piece_code.starts_with('w') {
                    Color::White
                } else {
                    Color::Rgb(30, 38, 80)
                };

                let piece_height = piece_str.lines().count() as u16;
                let top_padding = (cell_area.height.saturating_sub(piece_height)) / 2;

                let piece_rect = Rect {
                    x: cell_area.x,
                    y: cell_area.y + top_padding,
                    width: cell_area.width,
                    height: piece_height,
                };

                Paragraph::new(piece_str)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(fg_color).bg(bg_color))
                    .render(piece_rect, buf);
            }
        }
    }
}

impl<'a> Widget for Menu<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let total_height = 10 + 3 + 17 + 2 + 3; // Title(11) + Sp(3) + Modes(17) + Sp(2) + Hint(3)
        let main_layout = Layout::vertical([
            Constraint::Length(total_height),
        ])
            .flex(ratatui::layout::Flex::Center)
            .split(area);

        let menu_area = main_layout[0];

        let content_layout = Layout::vertical([
            Constraint::Length(10), // Title Area (Title + Creator)
            Constraint::Length(3),  // Spacer
            Constraint::Length(17), // Modes
            Constraint::Length(2),  // Spacer
            Constraint::Length(3),  // Hint Panel
        ])
            .split(menu_area);

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

        let modes_layout = Layout::horizontal([
            Constraint::Length(60), // Left mode column
            Constraint::Length(0),  // Center spacer
            Constraint::Length(60), // Right mode column
        ])
            .flex(ratatui::layout::Flex::Center)
            .split(content_layout[2]);

        // Normal Mode Preview
        let normal_area = modes_layout[0];
        let normal_content = Layout::vertical([
            Constraint::Length(10), // Previews (2 rows)
            Constraint::Length(1),  // Spacer
            Constraint::Length(6),  // Button area
        ])
            .flex(ratatui::layout::Flex::Center)
            .split(normal_area);

        let normal_preview_area = Layout::horizontal([Constraint::Length(60)])
            .flex(ratatui::layout::Flex::Center)
            .split(normal_content[0])[0];

        let preview_rows = Layout::vertical([Constraint::Length(5), Constraint::Length(5)])
            .split(normal_preview_area);

        self.render_preview(preview_rows[0], buf, &["br", "bn", "bb", "bq", "bk", "bb", "bn", "br"], false);
        self.render_preview(preview_rows[1], buf, &["wr", "wn", "wb", "wq", "wk", "wb", "wn", "wr"], true);

        let normal_bg = if self.states[0].pressed {
            Color::Red
        } else if self.states[0].focused {
            Color::Green
        } else {
            Color::Rgb(103, 115, 122)
        };

        let mut normal_style = ButtonStyle::default();
        normal_style.unfocused_bg = normal_bg;
        normal_style.focused_bg = normal_bg;

        let btn_normal_area = Layout::horizontal([Constraint::Length(50)]) // Wider for Artbox text
            .flex(ratatui::layout::Flex::Center)
            .split(normal_content[2])[0];

        Button::new("", &self.states[0]) // Empty label for render Artbox on top
            .style(normal_style)
            .render(btn_normal_area, buf);

        let normal_grid = Renderer::default()
            .with_alignment(ArtAlignment::Center)
            .render_grid("Normal", btn_normal_area.width, btn_normal_area.height)
            .unwrap();

        for (y, row) in normal_grid.chars.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let cell_buf = &mut buf[(btn_normal_area.x + x as u16, btn_normal_area.y + y as u16)];
                cell_buf.set_bg(normal_bg);
                if cell.ch != ' ' {
                    cell_buf.set_char(cell.ch).set_fg(Color::White);
                }
            }
        }

        // Fischer Mode Preview
        let fischer_area = modes_layout[2];
        let fischer_content = Layout::vertical([
            Constraint::Length(10), // Previews
            Constraint::Length(1),  // Spacer
            Constraint::Length(6),  // Button area
        ])
            .flex(ratatui::layout::Flex::Center)
            .split(fischer_area);

        let fischer_preview_area = Layout::horizontal([Constraint::Length(60)])
            .flex(ratatui::layout::Flex::Center)
            .split(fischer_content[0])[0];

        let f_preview_rows = Layout::vertical([Constraint::Length(5), Constraint::Length(5)])
            .split(fischer_preview_area);

        // A sample Fischer layout
        let f_row = &["bb", "bq", "br", "bk", "bn", "bb", "br", "bn"];
        let fw_row = &["wb", "wq", "wr", "wk", "wn", "wb", "wr", "wn"];

        self.render_preview(f_preview_rows[0], buf, f_row, false);
        self.render_preview(f_preview_rows[1], buf, fw_row, true);

        let fischer_bg = if self.states[1].pressed {
            Color::Red
        } else if self.states[1].focused {
            Color::Green
        } else {
            Color::Rgb(103, 115, 122)
        };

        let mut fischer_style = ButtonStyle::default();
        fischer_style.unfocused_bg = fischer_bg;
        fischer_style.focused_bg = fischer_bg;

        let btn_fischer_area = Layout::horizontal([Constraint::Length(50)])
            .flex(ratatui::layout::Flex::Center)
            .split(fischer_content[2])[0];

        Button::new("", &self.states[1])
            .style(fischer_style)
            .render(btn_fischer_area, buf);

        let fischer_grid = Renderer::default()
            .with_alignment(ArtAlignment::Center)
            .render_grid("Fischer", btn_fischer_area.width, btn_fischer_area.height)
            .unwrap();

        for (y, row) in fischer_grid.chars.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let cell_buf = &mut buf[(btn_fischer_area.x + x as u16, btn_fischer_area.y + y as u16)];
                cell_buf.set_bg(fischer_bg);
                if cell.ch != ' ' {
                    cell_buf.set_char(cell.ch).set_fg(Color::White);
                }
            }
        }

        // Hint Panel
        let hint_area = Layout::horizontal([Constraint::Length(40)])
            .flex(ratatui::layout::Flex::Center)
            .split(content_layout[4])[0];
        HintPanel.render(hint_area, buf);
    }
}
