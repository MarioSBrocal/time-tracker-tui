# Time Tracker TUI

Hey! Welcome to my personal time tracking tool.

I built this because I needed a simple way to track my work hours directly from the terminal and calculate intervals. I didn't want to use bloated web apps, pay for subscriptions, or deal with electron-based desktop apps just to log when I clock in and out. I wanted something that lives in my terminal, opens instantly, and stays out of my way.

So, I built this in Rust (also learn more about the language, MY FIRST RATATUI APP!).

## Why this exists (Features)

- **Terminal Native:** A clean TUI (Terminal User Interface) that you can open from anywhere.
- **Manage Periods:** You can register, edit, and delete time periods easily.
- **Visualize Data:** View your logged sessions directly in the terminal.
- **Calculate Totals:** Calculate the total number of hours between two specific dates by summing up all the registered intervals.
- **Local & Private:** Everything is saved to a local SQLite database on your machine (specifically in `~/.local/share/timetrackertui/`). No clouds, no accounts.
- **Crash-Free (Mostly):** I was pretty strict with the codebase. It uses zero `unwrap()` or `panic!()` macros, so it should handle errors gracefully instead of crashing your terminal (since I'm still learning, I would appreciate any bug reports if you find anything!).

## Installation

If you have Rust and Cargo installed, you can grab the latest version directly from this repo:

```bash
cargo install --git https://github.com/MarioSBrocal/time-tracker-tui.git
```

_(Note: Once installed, make sure your terminal's `$PATH` includes `~/.cargo/bin`!)_

## Updating

To keep the app updated when I push new changes, I highly recommend using `cargo-update`:

1. Install the updater: `cargo install cargo-update`
2. Update the tracker: `cargo install-update -a -g`

---

_Built for my own daily use, but feel free to fork it, use it, or open an issue if something breaks. Happy tracking!_
