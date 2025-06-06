# Note CLI
`note` is a command-line tool to automize my journaling.
## Features
- Configure the notes folder location.
- Display or create notes for the current day, week, or month.
## Dev
- add day week month last to check for last existing
- add day week month from-last to copy last into the current one
- add sync that runs git deploy merge
## Installation
### Prerequisites
- [Rust and Cargo](https://www.rust-lang.org/tools/install) must be installed on your system.
### Using Cargo
To install `note`, run the following command:
```bash
cargo install --git <repository-url>
```
Replace `<repository-url>` with the URL of this repo.
### Completions
To generate a completion script for a specific shell, you can run:
```bash
note generate-completions --shell fish > ~/.config/fish/completions/note.fish
```

