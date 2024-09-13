use crossterm::{
    event::{
        read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, Event,
    },
    execute,
    terminal::ClearType,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    crossterm::{
        self,
        event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
        terminal::Clear,
    },
    prelude::*,
    widgets::Paragraph,
    Terminal,
};
use std::{
    fmt::Write,
    io::{stderr, stdout, Result},
    thread::sleep,
    time::Duration,
};

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(
        std::io::stdout(),
        EnableBracketedPaste,
        EnableFocusChange,
        EnableMouseCapture,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES),
        // Clear(ClearType::All),
    )?;
    let mut capture_fixed = false;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;

    let mut scrollpos: usize = 0;
    let mut event_id = 0;
    let mut delta = 2;

    // `read()` blocks until an `Event` is available
    while let Ok(event) = read() {
        if !capture_fixed {
            execute!(std::io::stdout(), DisableMouseCapture);
            capture_fixed = true;
            execute!(std::io::stdout(), Clear(ClearType::All));
            println!("            ");
        }

        event_id += 1;
        let mut contents = String::new();

        write!(contents, "{:?}", event).unwrap();
        let size = terminal.size().unwrap();

        match event {
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(event) => {
                if event.code == ratatui::crossterm::event::KeyCode::Char('q') {
                    break;
                }

                match event.code {
                    ratatui::crossterm::event::KeyCode::Up => {
                        // if scrollpos != 0 {
                        scrollpos = scrollpos.saturating_sub(delta);
                        //     scrollpos -= 1;
                        // }
                    }
                    ratatui::crossterm::event::KeyCode::Down => {
                        scrollpos += delta;

                        if (scrollpos + size.height as usize) > 100 {
                            scrollpos = 100 - size.height as usize + 1;
                        }
                    }
                    _ => {}
                }
            }
            Event::Mouse(event) => match event.kind {
                ratatui::crossterm::event::MouseEventKind::ScrollDown => {
                    scrollpos += 1;
                }
                ratatui::crossterm::event::MouseEventKind::ScrollUp => {
                    scrollpos -= 1;
                }
                _ => {}
            },
            Event::Paste(data) => {
                // println!("{:?}", data)
            }
            Event::Resize(width, height) => {
                // println!("New size {}x{}", width, height)
            }
            event => {
                // println!("other: {:?}", event)
            }
        }

        // println!("Top Scroll: {}", scrollpos);
        terminal.draw(|f| {
            f.render_widget(
                Paragraph::new(format!(
                    "Scroll: {} - id: {event_id} - {contents}",
                    scrollpos
                ))
                .yellow()
                .on_black(),
                Rect::new(0, size.height - 1, size.width, 1),
            );
            // f.render_widget(
            //     Paragraph::new(format!("Scroll: {}", scrollpos)).blue(),
            //     Rect::new(size.width - 20, size.height - 1, 20, 1),
            // );
            for (idx, y) in (0..100)
                .cycle()
                .skip(scrollpos)
                .take(size.height.saturating_sub(1) as usize)
                .enumerate()
            {
                f.render_widget(
                    Paragraph::new(format!("Item {y}")),
                    Rect::new(0, idx as _, 20, 1),
                );
            }
        })?;
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    execute!(
        std::io::stdout(),
        DisableBracketedPaste,
        DisableFocusChange,
        DisableMouseCapture,
        // Clear(ClearType::All),
    )?;

    Ok(())
}
