mod pomodoro;
mod controller;
mod presentation;
mod util;
use crate::util::event::{Event, Events};
use std::{error::Error, io};
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

use controller::handle_input;
use pomodoro::{Session};
use presentation::{Presentation};


fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();
    let mut session = Session::new();
    while session.is_on() {
        session.update();
        let pres = Presentation::new(&session);
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().style(Style::default());
            f.render_widget(block, size);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints(
                    [
                        Constraint::Percentage(80),
                    ]
                    .as_ref(),
                )
                .split(size);
            let create_block = |title| {
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default())
                    .title(Span::styled(
                        title,
                        Style::default().add_modifier(Modifier::BOLD),
                    ))
            };
            let block_texts = vec![
                Spans::from(Span::styled(pres.time_left, Style::default())),
                Spans::from(Span::styled(pres.phase, Style::default())),
                Spans::from(Span::styled(pres.tomatoes_done, Style::default())),
                Spans::from(Span::styled(pres.paused, Style::default())),
            ];
            let paragraph = Paragraph::new(block_texts.clone())
                .style(Style::default())
                .block(create_block("Pomodoro"))
                .alignment(Alignment::Left);
            f.render_widget(paragraph, chunks[0]);
        })?;
        handle_input(&events, &mut session);
    }
    Ok(())
}
