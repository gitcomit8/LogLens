# LogLens ⚡

A blazing-fast, interactive TUI for analyzing logs and structured data directly in your terminal.

![Demo GIF of LogLens in action, showing a log file being piped into the tool and an interactive dashboard appearing with stats and a filterable list.](https://example.com/loglens_demo.gif)

Stop juggling `grep`, `awk`, `sed`, and `jq`. `LogLens` provides an instant, zero-configuration overview of your data, allowing you to find what matters, faster.

---

### Features

* **🚀 Blazing Fast:** Built in Rust to handle large files and streams with minimal overhead.
* **🧠 Smart Format Detection:** Automatically detects and parses common formats like JSON, Nginx access logs, and Apache logs on the fly.
* **📊 Live Summary Stats:** Get an instant overview of your data, such as HTTP status code counts, log level distribution (INFO, WARN, ERROR), and more.
* **🔍 Interactive Filtering:** Instantly filter the entire dataset by typing a query.
* **📦 Single Binary:** A single, cross-platform executable with no dependencies.

---

### Installation

#### From GitHub Releases

Download the appropriate binary for your OS and architecture from the [Releases page](https://github.com/gitcomit8/loglens/releases), unzip it, and place it in your `$PATH`.

---

### Usage

`LogLens` is designed to be used with pipes, just like your favorite command-line tools.

#### Analyze a static log file:

```bash
cat /var/log/nginx/access.log | loglens
```

#### Analyze a JSON API response:

```bash
curl -s "[https://api.github.com/repos/charmbracelet/bubbletea/commits](https://api.github.com/repos/charmbracelet/bubbletea/commits)" | loglens
```

#### Live-stream logs from a running application:

```bash
tail -f /var/log/my-app/app.log | loglens
```

---

### Roadmap

* [ ] Support for more log formats (systemd journal, syslog, etc.).
* [ ] The ability to connect directly to data sources (Elasticsearch, Loki).
* [ ] YAML and XML parsing.
* [ ] Configurable alerts and webhooks.

---

### License

This project is licensed under the **MIT License**.
