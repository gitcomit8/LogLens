use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};
use ratatui::{Frame, Terminal};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::io;

#[derive(Debug, Deserialize)]
struct ParsedLine {
    level: Option<String>,
    message: Option<String>,
    #[serde(flatten)]
    rest: BTreeMap<String, Value>,
}
struct App {
    should_quit: bool,
    items: Vec<ParsedLine>,
}
fn main() -> io::Result<()> {
    //----- DATA INGESTION -----
    let stdin = io::stdin();
    let mut items = Vec::new();
    for line in stdin.lines() {
        if let Ok(l) = line {
            if let Ok(parsed) = serde_json::from_str::<ParsedLine>(&l) {
                items.push(parsed);
            }
        }
    }
    //----- TUI SETUP -----
    enable_raw_mode()?; //Puts terminal to handle key presses directly
    let mut stdout = io::stdout();
    //Switches to alternate screen buffer to
    // prevent messing up shell history
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //---- APP CREATION & RUNNING
    let app = App {
        should_quit: false,
        items,
    };
    run_tui(&mut terminal, app)?;

    //----- TUI RESTORATION -----
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
fn ui(frame: &mut Frame, app: &App) {
    //Define the layout
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)
        .split(frame.area());

    //----- TABLE CREATION -----
    let header_cells = ["Level", "Message", "Other Data"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().bold()));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    //Convert ParsedLine items into Rows
    let rows = app.items.iter().map(|item| {
        let level = item.level.as_deref().unwrap_or("-");
        let message = item.message.as_deref().unwrap_or("-");
        //format the rest of JSON data
        let rest = item
            .rest
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v.to_string().trim_matches('"')))
            .collect::<Vec<_>>()
            .join(", ");

        let cells = vec![Cell::from(level), Cell::from(message), Cell::from(rest)];
        Row::new(cells).height(1)
    });

    //Define column widths
    let widths = [
        Constraint::Length(8),
        Constraint::Length(50),
        Constraint::Percentage(100),
    ];

    //Create table widget
    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Logs"));

    //Render the table
    frame.render_widget(table, rects[0]);
}
