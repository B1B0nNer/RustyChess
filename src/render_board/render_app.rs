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
use crate::render_board::history_panel::HistoryPanel;
use crate::render_board::hint_panel::HintPanel;
use crate::render_board::captured_panel::CapturedPanel;
use crate::Game;

pub fn run_game(game: &mut Game) -> Result<(), Box<dyn Error>> {
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
            let title_text = "RUSTY CHESS";
            let title = Paragraph::new(title_text)
                .centered()
                .style(Style::default().add_modifier(Modifier::BOLD))
                .block(title_block);

            // Center the title block horizontally as well
            let title_layout = Layout::horizontal([Constraint::Length(40)])
                .flex(Flex::Center)
                .split(layout[0]);

            f.render_widget(title, title_layout[0]);

            // Main horizontal layout for board and info panel
            let main_layout = Layout::horizontal([
                Constraint::Fill(1),              // Left spacer
                Constraint::Length(25),           // Captured panel (left side)
                Constraint::Length(board_width),  // Board
                Constraint::Length(25),           // Info panel (right side)
                Constraint::Fill(1),              // Right spacer
            ])
            .split(layout[1]);

            let captured_area = main_layout[1];
            let board_area = main_layout[2];
            let info_area = main_layout[3];
            
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

            let grid = render_board::render_board(&game.board, &button_states, &game.valid_moves);
            f.render_widget(grid, board_area);

            // Render Captured Pieces Panel
            let captured_panel = CapturedPanel {
                captured_by_white: &game.captured_by_white,
                captured_by_black: &game.captured_by_black,
            };
            f.render_widget(captured_panel, captured_area);

            // Render Info Panel (Turn indicator and History)
            let info_panel = HistoryPanel {
                turn: game.turn,
                history: &game.history,
            };
            f.render_widget(info_panel, info_area);

            let hint_layout = Layout::horizontal([Constraint::Length(28)])
                .flex(Flex::Center)
                .split(layout[2]);

            f.render_widget(HintPanel, hint_layout[0]);
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
                            let r = (i / 8) as i8;
                            let c = (i % 8) as i8;
                            let content = game.board[r as usize][c as usize];

                            match mouse.kind {
                                MouseEventKind::Down(MouseButton::Left) => {
                                    button_states[i].pressed = true;
                                            
                                    let is_valid_move = game.valid_moves.contains(&(r, c));
                                            
                                    if is_valid_move {
                                        game.move_selected_piece(r, c);
                                    } else if !content.is_empty() && content != "hint" {
                                        game.select_figure(r, c);
                                    }
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
