use artbox::Renderer;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect,},
    widgets::{Widget},
};
use ratatui_core::layout::Direction;
use ratatui_core::style::Color;
use ratatui_interact::components::{Button, ButtonState, ButtonStyle};
use strum::IntoEnumIterator;
use crate::render_board::menu::text_render::text_render;
use crate::render_board::panels::hint_panel::HintPanel;
use super::title::render_title;
use super::super::super::game::game::TimeMode;

pub struct TimeMenu<'a> {
    pub states: &'a [ButtonState; 9],
}

pub fn get_time_menu_button_areas(area: Rect) -> [Rect; 9] {
    // These must match the constants used in the render() function exactly
    let btn_height = 10;
    let btn_width = 27;

    let content_layout = Layout::vertical([
        Constraint::Length(10),         // Title
        Constraint::Length(2),          // Spacer
        Constraint::Length(btn_height), // Row 1 [2]
        Constraint::Length(1),          // Spacer [3]
        Constraint::Length(btn_height), // Row 2 [4]
        Constraint::Length(1),          // Spacer [5]
        Constraint::Length(btn_height), // Row 3 [6]
        Constraint::Length(2),          // Spacer [7]
        Constraint::Length(3),          // Hint [8]
    ])
        .flex(ratatui::layout::Flex::Center)
        .split(area);

    let horizontal_constraints = [
        Constraint::Length(btn_width),
        Constraint::Length(2), // Horizontal Spacer
        Constraint::Length(btn_width),
        Constraint::Length(2), // Horizontal Spacer
        Constraint::Length(btn_width),
    ];

    let mut buttons = [Rect::default(); 9];

    for i in 0..9 {
        let row_idx = 2 + (i / 3) * 2;
        let row_area = content_layout[row_idx];

        let row_columns = Layout::horizontal(horizontal_constraints)
            .flex(ratatui::layout::Flex::Center)
            .split(row_area);

        // Indices 0, 2, 4 are the actual buttons (1 and 3 are spacers)
        buttons[i] = row_columns[(i % 3) * 2];
    }

    buttons
}

impl<'a> Widget for TimeMenu<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // 1. Calculate a more reasonable height for terminal 'squares'
        // In TUIs, a 3:1 width-to-height ratio (e.g., 15 wide, 5 high)
        // usually looks more like a natural button.
        let btn_height = 10;
        let btn_width = 27;

        let content_layout = Layout::vertical([
            Constraint::Length(10), // Title
            Constraint::Length(2),  // Spacer
            Constraint::Length(btn_height), // Row 1
            Constraint::Length(1),          // Spacer
            Constraint::Length(btn_height), // Row 2
            Constraint::Length(1),          // Spacer
            Constraint::Length(btn_height), // Row 3
            Constraint::Length(2),          // Spacer
            Constraint::Length(3),          // Hint
        ])
            .flex(ratatui::layout::Flex::Center) // Keeps the whole menu centered vertically
            .split(area);

        render_title(&content_layout, buf);

        let horizontal_constraints = [
            Constraint::Length(btn_width),
            Constraint::Length(2), // Horizontal Spacer
            Constraint::Length(btn_width),
            Constraint::Length(2), // Horizontal Spacer
            Constraint::Length(btn_width),
        ];

        for (i, (state, mode)) in self.states.iter().zip(TimeMode::iter()).enumerate() {
            let row_idx = 2 + (i / 3) * 2; // Maps to content_layout indices 2, 4, 6
            let row_area = content_layout[row_idx];

            let row_columns = Layout::horizontal(horizontal_constraints)
                .flex(ratatui::layout::Flex::Center)
                .split(row_area);

            let btn_area = row_columns[(i % 3) * 2];

            // --- Dynamic Styling ---
            let (bg, fg) = if state.pressed {
                (Color::Red, Color::White)
            } else if state.focused {
                (Color::Green, Color::Black)
            } else {
                (Color::Rgb(60, 60, 60), Color::Gray) // Darker gray for cleaner look
            };

            let mut style = ButtonStyle::default();
            style.unfocused_bg = bg;
            style.focused_bg = bg;

            // Render background
            Button::new("", state)
                .style(style)
                .render(btn_area, buf);

            let mode_text = match mode {
                // Remove the manual \n and extra spaces
                TimeMode::Unlimited => " No Time ".to_string(),
                TimeMode::OneMinute => " 1 Min ".to_string(),
                _ => mode.to_string(),
            };

            if mode == TimeMode::Unlimited {
                // Split the button area into top and bottom halves
                let split_btn = Layout::vertical([
                    Constraint::Percentage(45),
                    Constraint::Percentage(5),
                    Constraint::Percentage(50),
                ]).split(btn_area);

                let lines = ["No", "", "Time"];
                for (i, line) in lines.iter().enumerate() {
                    if let Ok(grid) = Renderer::default()
                        .with_alignment(artbox::Alignment::Center)
                        .render_grid(line, split_btn[i].width, split_btn[i].height)
                    {
                        text_render(grid, split_btn[i], bg, buf);
                    }
                }
            } else {
                // Standard rendering for everything else
                if let Ok(grid) = Renderer::default()
                    .with_alignment(artbox::Alignment::Center)
                    .render_grid(&mode_text, btn_area.width, btn_area.height)
                {
                    text_render(grid, btn_area, bg, buf);
                }
            }
        }

        // Hint Panel centering
        let hint_area = Layout::horizontal([Constraint::Length(50)])
            .flex(ratatui::layout::Flex::Center)
            .split(content_layout[8])[0];
        HintPanel.render(hint_area, buf);
    }
}