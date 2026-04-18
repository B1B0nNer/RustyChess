use std::error::Error;
use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Flex, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Paragraph},
    Terminal,
};
use ratatui_interact::components::ButtonState;

use crate::render_board::render_board;

pub fn run_game(chess_board: &Vec<Vec<&'static str>>) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut button_states: [ButtonState; 64] = std::array::from_fn(|_| ButtonState::default());
    let mut cell_areas = [Rect::default(); 64];

    loop {
        terminal.draw(|f| {
            let area = f.area();

            // Calculate the size of the board: 8 cols * 12 widths = 96, 8 rows * 5 heights = 40
            let board_width = 8 * 12;
            let board_height = 8 * 5;

            let layout = Layout::vertical([
                Constraint::Length(3),            // Title
                Constraint::Length(board_height), // Board
                Constraint::Length(3),            // Hint
            ])
            .flex(Flex::Center)
            .split(area);

            let title_block = Block::bordered()
                .border_type(ratatui::widgets::BorderType::Thick)
                .style(Style::default().fg(Color::Yellow));
            let title = Paragraph::new("RUSTY CHESS")
                .centered()
                .style(Style::default().add_modifier(Modifier::BOLD))
                .block(title_block);

            // Center the title block horizontally as well
            let title_layout = Layout::horizontal([Constraint::Length(40)])
                .flex(Flex::Center)
                .split(layout[0]);

            f.render_widget(title, title_layout[0]);

            let sub_layout = Layout::horizontal([Constraint::Length(board_width)])
                .flex(Flex::Center)
                .split(layout[1]);

            let board_area = sub_layout[0];
            
            // Calculate individual cell areas for mouse hit testing
            let col_constraints = (0..8).map(|_| Constraint::Length(12));
            let row_constraints = (0..8).map(|_| Constraint::Length(5));
            let horizontal = Layout::horizontal(col_constraints);
            let vertical = Layout::vertical(row_constraints);
            let rows = vertical.split(board_area);
            for (r, row_rect) in rows.iter().enumerate() {
                let cols = horizontal.split(*row_rect);
                for (c, col_rect) in cols.iter().enumerate() {
                    cell_areas[r * 8 + c] = *col_rect;
                }
            }

            let grid = render_board::render_board(chess_board, &button_states);
            f.render_widget(grid, board_area);

            let hint_block = Block::bordered()
                .border_type(ratatui::widgets::BorderType::Plain)
                .style(Style::default().fg(Color::Gray));
            let hint = Paragraph::new("Press q to close")
                .centered()
                .block(hint_block);

            let hint_layout = Layout::horizontal([Constraint::Length(28)])
                .flex(Flex::Center)
                .split(layout[2]);

            f.render_widget(hint, hint_layout[0]);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => {
                    if let KeyCode::Char('q') = key.code {
                        break;
                    }
                }
                Event::Mouse(mouse) => {
                    let (column, row) = (mouse.column, mouse.row);
                    for i in 0..64 {
                        let area = cell_areas[i];
                        let is_inside = column >= area.x 
                            && column < area.x + area.width 
                            && row >= area.y 
                            && row < area.y + area.height;
                        
                        if is_inside {
                            let r = i / 8;
                            let c = i % 8;
                            let has_piece = !chess_board[r][c].is_empty();

                            if has_piece {
                                match mouse.kind {
                                    MouseEventKind::Down(MouseButton::Left) => {
                                        button_states[i].pressed = true;
                                    }
                                    MouseEventKind::Up(MouseButton::Left) => {
                                        button_states[i].focused = true;
                                    }
                                    MouseEventKind::Moved => {
                                        button_states[i].focused = true;
                                    }
                                    _ => {}
                                }
                            } else {
                                button_states[i].focused = false;
                                button_states[i].pressed = false;
                            }
                        } else {
                            button_states[i].focused = false;
                            button_states[i].pressed = false;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
