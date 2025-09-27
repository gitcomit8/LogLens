use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{
	disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::prelude::Direction;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{Frame, Terminal};
use std::io;

struct App {
    should_quit: bool,
}
fn main() -> io::Result<()> {
    enable_raw_mode()?; //Puts terminal to handle key presses directly
    let mut stdout = io::stdout();
    //Switches to alternate screen buffer to
    // prevent messing up shell history
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //Create app and run it
    let app = App { should_quit: false };
    run_tui(&mut terminal, app)?;

    //Restore terminal to original state
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}

//Main TUI loop
fn run_tui<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        //draw the ui
        terminal.draw(|f| ui(f, &app))?;

        //handle events like key presses
        // poll with a timeout of 250ms
        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    app.should_quit = true;
                }
            }
        }
        if app.should_quit {
            return Ok(());
        }
    }
}

//this fn defines and draws the widgets
fn ui(frame: &mut Frame, _app: &App) {
    //Create main layout that splits the screen vertically
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.area());

    //Create paragraph widget
    let text = Paragraph::new("Hello, LogLens!\n Press 'q' to quit.")
        .block(Block::default().borders(Borders::ALL).title("LogLens"))
        .alignment(Alignment::Center);

    //Render the widget in the first chunk of the layout
    frame.render_widget(text, chunks[0]);
}
