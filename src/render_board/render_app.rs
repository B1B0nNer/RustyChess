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
    widgets::{Block, BorderType, Paragraph},
    Terminal,
};
use ratatui_interact::components::ButtonState;

use crate::render_board::render_board::{self, get_ascii_art};
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
    let mut replay_button_state = ButtonState::default();
    let mut cell_areas = [Rect::default(); 64];
    let mut replay_button_area = Rect::default();
    let mut promotion_areas = Vec::new();

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
                Constraint::Length(3),            // Replay Button Slot
            ])
            .flex(Flex::Center)
            .split(area);

            let mut title_color = Color::Yellow;
            let mut title_text = "RUSTY CHESS".to_string();
            
            if game.is_checkmate {
                title_color = Color::Red;
                title_text = format!("CHECKMATE - {} LOSES", if game.turn == 'w' { "WHITE" } else { "BLACK" });
            } else if game.is_stalemate {
                title_color = Color::Yellow;
                title_text = "STALEMATE - DRAW".to_string();
            } else if game.is_check {
                title_color = Color::LightRed;
                title_text = format!("CHECK - {}'S KING", if game.turn == 'w' { "WHITE" } else { "BLACK" });
            }

            let title_block = Block::bordered()
                .border_type(BorderType::Thick)
                .style(Style::default().fg(title_color));
            
            let title = Paragraph::new(title_text)
                .centered()
                .style(Style::default().add_modifier(Modifier::BOLD))
                .block(title_block);

            // Center the title block horizontally as well
            let title_layout = Layout::horizontal([Constraint::Length(60)])
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
                is_check: game.is_check,
                is_checkmate: game.is_checkmate,
                is_stalemate: game.is_stalemate,
            };
            f.render_widget(info_panel, info_area);

            let hint_layout = Layout::horizontal([Constraint::Length(28)])
                .flex(Flex::Center)
                .split(layout[2]);

            f.render_widget(HintPanel, hint_layout[0]);

            // Render Replay Button if game is over
            if game.is_checkmate || game.is_stalemate {
                let replay_layout = Layout::horizontal([Constraint::Length(12)])
                    .flex(Flex::Center)
                    .split(layout[3]);
                
                replay_button_area = replay_layout[0];
                
                let mut replay_style = Style::default().fg(Color::Black).bg(Color::White);
                if replay_button_state.pressed {
                    replay_style = replay_style.bg(Color::Gray);
                } else if replay_button_state.focused {
                    replay_style = replay_style.bg(Color::LightCyan);
                }

                let replay_button = Paragraph::new(" REPLAY ")
                    .centered()
                    .block(Block::bordered().style(replay_style));
                
                f.render_widget(replay_button, replay_button_area);
            } else {
                replay_button_area = Rect::default();
            }

            // Render Promotion Overlay
            if let Some(_) = game.promotion {
                promotion_areas.clear();
                let dead_figures = if game.turn == 'w' {
                    &game.captured_by_black
                } else {
                    &game.captured_by_white
                };

                let overlay_area = Layout::vertical([Constraint::Length(9)])
                    .flex(Flex::Center)
                    .split(area);
                
                let overlay_box = Layout::horizontal([Constraint::Length(70)])
                    .flex(Flex::Center)
                    .split(overlay_area[0])[0];

                f.render_widget(ratatui::widgets::Clear, overlay_box);
                f.render_widget(Block::bordered().title(" SELECT PROMOTION ").style(Style::default().bg(Color::Blue).fg(Color::White)), overlay_box);

                if dead_figures.is_empty() {
                    let msg = Paragraph::new("No captured pieces to promote to!\n(Wait, this shouldn't happen in this mode?)").centered();
                    f.render_widget(msg, overlay_box.inner(ratatui::layout::Margin { horizontal: 1, vertical: 2 }));
                    
                } else {
                    let item_width = 10;
                    let items_count = dead_figures.len();
                    
                    let items_layout = Layout::horizontal(vec![Constraint::Length(item_width); items_count])
                        .flex(Flex::Center)
                        .split(overlay_box.inner(ratatui::layout::Margin { horizontal: 1, vertical: 1 }));

                    for (idx, piece_code) in dead_figures.iter().enumerate() {
                        let item_area = items_layout[idx];
                        promotion_areas.push((item_area, idx));
                        
                        let fg_color = if piece_code.starts_with('w') {
                            Color::Rgb(242, 242, 209)
                        } else {
                            Color::Black
                        };

                        let piece_str = get_ascii_art(piece_code);
                        let piece_height = piece_str.lines().count() as u16;
                        f.render_widget(
                            Block::bordered().style(Style::default().fg(Color::White)),
                            item_area
                        );

                        if !piece_str.is_empty() {
                            let piece_layout = Layout::vertical([
                                Constraint::Length(1), // Top border
                                Constraint::Fill(1),   // Flexible space
                                Constraint::Length(piece_height), // The art
                                Constraint::Fill(1),   // Flexible space
                                Constraint::Length(1), // Bottom border
                            ]).split(item_area);

                            f.render_widget(
                                Paragraph::new(piece_str)
                                    .centered()
                                    .style(Style::default().fg(fg_color)),
                                piece_layout[2]
                            );
                        }
                    }
                }
            }
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

                    // Handle Promotion Selection
                    if game.promotion.is_some() {
                        if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                            for (area, idx) in &promotion_areas {
                                if column >= area.x && column < area.x + area.width
                                    && row >= area.y && row < area.y + area.height {
                                    game.promote_pawn(*idx);
                                    break;
                                }
                            }
                        }
                        continue;
                    }

                    // Check Replay Button
                    if !replay_button_area.is_empty() {
                        let is_inside = column >= replay_button_area.x 
                            && column < replay_button_area.x + replay_button_area.width 
                            && row >= replay_button_area.y 
                            && row < replay_button_area.y + replay_button_area.height;
                        
                        if is_inside {
                            match mouse.kind {
                                MouseEventKind::Down(MouseButton::Left) => {
                                    replay_button_state.pressed = true;
                                    game.reset();
                                }
                                MouseEventKind::Up(MouseButton::Left) => {
                                    replay_button_state.focused = true;
                                    replay_button_state.pressed = false;
                                }
                                MouseEventKind::Moved => {
                                    replay_button_state.focused = true;
                                }
                                _ => {}
                            }
                        } else {
                            replay_button_state.focused = false;
                            replay_button_state.pressed = false;
                        }
                    }

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

                            if game.is_checkmate || game.is_stalemate {
                                continue;
                            }

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
