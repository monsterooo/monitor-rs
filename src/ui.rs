use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Styled, Stylize},
    symbols,
    text::Text,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, RenderDirection, Sparkline},
    Frame,
};

use crate::app::App;

const LEFT_WIDTH: u16 = 30;
const RIGHT_WIDTH: u16 = 70;

///   ---------------------
///   |     |              |
///   | info|     chart    |
///   |     |              |
///   |     |              |
///   |     |              |
///   |     |              |
///   ----------------------

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    // frame.render_widget(
    //     Paragraph::new(format!(
    //         "This is a tui template.\n\
    //             Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
    //             Press left and right to increment and decrement the counter respectively.\n\
    //             Counter: {}",
    //         app.counter
    //     ))
    //     .block(
    //         Block::bordered()
    //             .title("Template")
    //             .title_alignment(Alignment::Center)
    //             .border_type(BorderType::Rounded),
    //     )
    //     .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    //     .centered(),
    //     frame.size(),
    // );

    let layout_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(LEFT_WIDTH),
            Constraint::Percentage(RIGHT_WIDTH),
        ])
        .split(frame.size());

    let left_block = Block::new()
        .border_set(symbols::border::PLAIN)
        .borders(Borders::TOP | Borders::BOTTOM | Borders::LEFT)
        .border_type(BorderType::Rounded)
        .title("Left");
    let info_text = Paragraph::new(Text::from(format!(
        "name: {}\ntotal memory: {}\nused memory: {}\nCPUs: {}",
        app.system_info.host_name,
        app.system_info.total_memory,
        app.system_info.used_memory,
        app.system_info.cpus
    )))
    .block(left_block)
    .alignment(Alignment::Center);
    // left
    frame.render_widget(info_text, layout_chunks[0]);

    // right
    frame.render_widget(
        Block::new()
            .border_set(symbols::border::Set {
                top_left: symbols::line::HORIZONTAL_DOWN,
                bottom_left: symbols::line::HORIZONTAL_UP,
                bottom_right: symbols::line::ROUNDED_BOTTOM_RIGHT,
                top_right: symbols::line::ROUNDED_TOP_RIGHT,
                ..symbols::border::PLAIN
            })
            .borders(Borders::ALL)
            .title("Right"),
        layout_chunks[1],
    );

    let monitor_memory = Sparkline::default()
        .data(&*app.system_info.monitor_memory)
        .direction(RenderDirection::LeftToRight);
    frame.render_widget(monitor_memory, layout_chunks[1]);
}
