### The 10-Day Plan

**Tech Stack:**
* **Language:** **Rust**
* **TUI Framework:** **`ratatui`** (for drawing UI) + **`crossterm`** (for terminal backend)
* **Packaging:** **`cargo`**

---

#### Phase 1: The Core Engine (Weekend 1: Sep 27-28)

**Day 1 (Today, Saturday): Environment & Ingestion**
* **Goal:** Create a Rust program that reads data piped into it.
* **Tasks:**
    1.  Create your new project: `cargo new loglens`.
    2.  Set up dependencies in `Cargo.toml`. You'll need `ratatui`, `crossterm`, and `serde_json` to start.
    3.  Write the core logic in `main.rs` to read from `std::io::stdin()` line by line and print it back out.
* **Success Metric:** You can run `cat somefile.log | cargo run` and see the content printed.

**Day 2 (Sunday): The Parser Engine**
* **Goal:** Turn raw text into structured data efficiently.
* **Tasks:**
    1.  Define a `ParsedLine` struct and derive `serde::Deserialize` on it.
    2.  Use the `serde_json` crate to write a function that takes a string slice and returns a `Result<ParsedLine, _>`.
* **Success Metric:** Your program now prints a structured, debug representation of each valid JSON line it reads.

---

#### Phase 2: Building the TUI (Weekdays: Sep 29 - Oct 3)

**Day 3 (Monday): The TUI Boilerplate**
* **Goal:** Set up the main application loop and draw a single widget.
* **Tasks:**
    1.  Create your main `App` struct which will hold the application state.
    2.  In `main.rs`, write the TUI boilerplate: enable raw mode, set up the `Terminal` with the `crossterm` backend, and create the main loop that draws to the screen and handles events.
    3.  Draw a simple `Paragraph` widget from `ratatui` that says "Hello, LogLens!".
* **Success Metric:** Your app runs, clears the screen, displays the message, and exits cleanly when you press 'q'.

**Day 4 (Tuesday): Displaying Data in a Table**
* **Goal:** Render the parsed log data in a scrollable table.
* **Tasks:**
    1.  Use the `ratatui::widgets::Table` widget.
    2.  Feed your `Vec<ParsedLine>` from your `App` struct into the table to render the rows.
    3.  Define the table headers and column constraints.
* **Success Metric:** When you pipe a log file, the TUI launches and displays the parsed data in a structured table.

**Day 5 (Wednesday): State Management & Scrolling**
* **Goal:** Implement scrolling for the table.
* **Tasks:**
    1.  Manage the scroll state yourself. Add fields to your `App` struct to track the selected row index (`TableState`).
    2.  In your event handling logic, modify this state when the user presses the up/down arrow keys.
    3.  Pass the `TableState` to the `table.render()` method in your draw loop.
* **Success Metric:** You can smoothly scroll up and down the list of logs.

**Day 6 (Thursday): Layouts & The Summary Dashboard**
* **Goal:** Create a multi-panel layout.
* **Tasks:**
    1.  Use `ratatui::layout::{Layout, Constraint, Direction}` to split your terminal screen into different rectangular areas.
    2.  Create a top area for a summary `Paragraph` widget and a main area for your `Table`.
    3.  Keep track of stats in your `App` struct and display them in the summary panel.
* **Success Metric:** The TUI has a clean, multi-panel layout showing stats and the main log view.

**Day 7 (Friday): The Killer Feature - Filtering**
* **Goal:** Implement live, keystroke-by-keystroke filtering.
* **Tasks:**
    1.  Add a `filter_query: String` and an `input_mode: Enum` to your `App` struct.
    2.  Handle keyboard input to append characters to the `filter_query` when in `InputMode::Editing`.
    3.  In your draw loop, create a temporary, filtered list of logs based on the query before rendering the table.
* **Success Metric:** The user can enter a filter mode, type a query, and see the table update in real-time.

---

#### Phase 3: Polish & Launch (Weekend 2: Oct 4-6)

**Day 8 (Saturday): The Second Parser & Polish**
* **Goal:** Add Nginx support and styling.
* **Tasks:**
    1.  Add the `regex` crate to your dependencies. Write the Nginx parser.
    2.  Use `ratatui::style::{Style, Color, Modifier}` to add color. Style rows based on log level (`ERROR` in red).
    3.  Refine the UI, adding borders and a help text panel.
* **Success Metric:** The app looks sharp and professionally styled.

**Day 9 (Sunday): Packaging & Documentation**
* **Goal:** Prepare for public release.
* **Tasks:**
    1.  Write the complete `README.md` file for your project.
    2.  Run `cargo build --release`. This creates a highly optimized, single binary in the `target/release/` directory.
    3.  Test the release binary to ensure it works as expected.
* **Success Metric:** You have a single, fast executable file ready for distribution.

**Day 10 (Monday): Launch Day!**
* **Goal:** SHIP IT. 🚀
* **Tasks:**
    1.  Create the public GitHub repository and push your code.
    2.  Create a `v0.1.0` release on GitHub and upload the compiled binary.
    3.  Post a GIF of your app on Hacker News, Reddit (/r/rust), or Twitter.
* **Success Metric:** Your project is live, public, and you've shared your work.
