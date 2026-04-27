use artbox::GridRendered;
use ratatui_core::layout::Rect;
use ratatui::style::Color;
use ratatui_core::buffer::Buffer;

pub fn text_render(grid: GridRendered, btn_area: Rect, bg: Color, buf: &mut Buffer) {
    // 1. Calculate actual dimensions of the generated ASCII grid
    let grid_height = grid.chars.len() as u16;
    let grid_width = grid.chars.first().map(|f| f.len()).unwrap_or(0) as u16;

    // 2. Calculate offsets to center the grid inside the button area
    let x_offset = if btn_area.width > grid_width { (btn_area.width - grid_width) / 2 } else { 0 };
    let y_offset = if btn_area.height > grid_height { (btn_area.height - grid_height) / 2 } else { 0 };

    for (y, row) in grid.chars.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let target_x = btn_area.x + x_offset + x as u16;
            let target_y = btn_area.y + y_offset + y as u16;

            // Safety check to stay inside the button and buffer bounds
            if target_x < btn_area.x + btn_area.width && target_y < btn_area.y + btn_area.height {
                let cell_buf = &mut buf[(target_x, target_y)];

                // Keep the button's background color
                cell_buf.set_bg(bg);

                // Only draw non-space characters
                if cell.ch != ' ' {
                    cell_buf.set_char(cell.ch).set_fg(Color::White);
                }
            }
        }
    }
}