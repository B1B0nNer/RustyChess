use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Widget, Paragraph},
};
use ratatui_interact::components::{Button, ButtonState, ButtonStyle};

pub fn get_ascii_art(code: &str) -> &'static str {
    match code {
        // White Pieces
        "wp" => " ▄█▄ \n ▀█▀ \n▄███▄", // Pawn
        "wr" => "█▄█▄█\n ███ \n▄███▄", // Rook
        "wh" => "▄███ \n  ██ \n▄████", // Knight
        "wb" => " ▄█▄ \n (█) \n▄███▄", // Bishop
        "wq" => "█▀█▀█\n ███ \n█████", // Queen
        "wk" => "█ █ █\n▀███▀\n▄███▄", // King
 
        // Black Pieces
        "bp" => " ▄█▄ \n ▀█▀ \n▄███▄", // Pawn
        "br" => "█▄█▄█\n ███ \n▄███▄", // Rook
        "bh" => "▄███ \n  ██ \n▄████", // Knight
        "bb" => " ▄█▄ \n (█) \n▄███▄", // Bishop
        "bq" => "█▀█▀█\n ███ \n█████", // Queen
        "bk" => "█ █ █\n▀███▀\n▄███▄", // King
 
        "hint" => " ▄▄▄ \n█   █\n ▀▀▀ ", // Move hint circle

        _ => "",
    }
}

pub struct Grid<'a> {
    pub cols: usize,
    pub rows: usize,
    pub board: &'a Vec<Vec<&'static str>>,
    pub states: &'a [ButtonState; 64],
    pub valid_moves: &'a Vec<(usize, usize)>,
}

impl<'a> Widget for Grid<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_constraints = (0..self.cols).map(|_| Constraint::Length(12));
        let row_constraints = (0..self.rows).map(|_| Constraint::Length(5));

        let horizontal = Layout::horizontal(col_constraints);
        let vertical = Layout::vertical(row_constraints);

        let rows_layout = vertical.split(area);
        let cells = rows_layout.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            let row = i / self.cols;
            let col = i % self.cols;
            let content = self.board[row][col];

            let is_dark = (row + col) % 2 != 0;
            let bg_color = if is_dark {
                Color::Rgb(112, 121, 207)
            } else {
                Color::Rgb(153, 173, 255)
            };

            let fg_color = if content.starts_with('w') {
                Color::Rgb(242, 242, 209)
            } else if content.starts_with('b') {
                Color::Black
            } else {
                Color::White
            };

            let is_attack_hint = self.valid_moves.contains(&(row, col)) && !content.is_empty() && content != "hint";

            let current_bg_color = if self.states[i].pressed {
                Color::Red
            } else if self.states[i].focused {
                Color::Green
            } else if content == "hint" {
                Color::Rgb(100, 200, 100) // Distinct color for hint cells
            } else if is_attack_hint {
                Color::Rgb(220, 50, 50) // Brighter red for attack targets
            } else {
                bg_color
            };

            let block = Block::default()
                .style(Style::default().bg(current_bg_color).fg(current_bg_color));

            block.render(cell, buf);

            let mut button_style = ButtonStyle::default();
            button_style.unfocused_fg = fg_color;
            button_style.unfocused_bg = current_bg_color;
            button_style.focused_fg = fg_color;
            button_style.focused_bg = Color::Green;
            button_style.pressed_fg = fg_color;
            button_style.pressed_bg = Color::Red;

            // Render the button (empty label since we render art on top or instead)
            Button::new("", &self.states[i])
                .style(button_style)
                .render(cell, buf);

            // 2. Render the piece art on top
            let piece_str = get_ascii_art(content);

            if !piece_str.is_empty() {
                let piece_height = piece_str.lines().count() as u16;
                let top_padding = (cell.height.saturating_sub(piece_height)) / 2;
                let bottom_padding = cell.height.saturating_sub(piece_height).saturating_sub(top_padding);

                let vertical_layout = Layout::vertical([
                    Constraint::Length(top_padding),
                    Constraint::Length(piece_height),
                    Constraint::Length(bottom_padding),
                ]).split(cell);

                let piece_widget = Paragraph::new(piece_str)
                    .alignment(ratatui::layout::Alignment::Center)
                    .style(Style::default().fg(fg_color).bg(current_bg_color));

                piece_widget.render(vertical_layout[1], buf);
            }
        }
    }
}

pub fn render_board<'a>(
    board: &'a Vec<Vec<&'static str>>, 
    states: &'a [ButtonState; 64],
    valid_moves: &'a Vec<(usize, usize)>,
) -> Grid<'a> {
    Grid {
        cols: 8,
        rows: 8,
        board,
        states,
        valid_moves,
    }
}