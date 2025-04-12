use chrono::{Datelike, Local};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::aot::{generate, Shell};
use config::{Config, ConfigError, File, FileFormat};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
#[derive(Debug, Deserialize, Serialize, Default)]
struct NoteConfig {
    data_location: String,
    // Add more configuration fields as needed
}
#[derive(Parser)]
#[command(name = "note")]
#[command(about = "A CLI tool for managing notes", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Configure the notes folder
    Config {
        /// Set the folder path
        #[arg(long)]
        folder: String,
    },
    /// Display index.md of the current year
    Index,
    /// Display a custom file of the current year
    Foo,
    /// Display or create day.md for the current date
    Day,
    /// Display or create week.md for the current week
    Week,
    /// Display or create month.md for the current month
    Month,
    /// Generate shell completions
    GenerateCompletions {
        /// The shell to generate completions for
        #[arg(long)]
        shell: Shell,
    },
}
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Config { folder } => {
            let mut config = load_config().unwrap_or_default();
            config.data_location = folder.clone();
            save_config(&config).expect("Failed to save configuration");
            println!("Notes folder set to: {}", folder);
        }
        Commands::Index => {
            display_file_for_current_year("index.md");
        }
        Commands::Foo => {
            display_file_for_current_year("foo.md");
        }
        Commands::Day => {
            handle_note("day.md", "template/day.md");
        }
        Commands::Week => {
            handle_note("week.md", "template/week.md");
        }
        Commands::Month => {
            handle_note("month.md", "template/month.md");
        }
        Commands::GenerateCompletions { shell } => {
            let mut cmd = Cli::command(); // Use the CommandFactory trait
            let bin_name = cmd.get_name().to_string();
            generate(*shell, &mut cmd, bin_name, &mut io::stdout());
        }
    }
}
fn get_config_path() -> PathBuf {
    let mut config_path = config_dir().expect("Could not find config directory");
    config_path.push("note");
    fs::create_dir_all(&config_path).expect("Could not create config directory");
    config_path.push("noterc");
    config_path
}
fn load_config() -> Result<NoteConfig, ConfigError> {
    let config_path = get_config_path();
    let settings = Config::builder()
        // Explicitly specify the format as TOML
        .add_source(File::from(config_path).format(FileFormat::Toml))
        .build()?;
    settings.try_deserialize()
}
fn save_config(config: &NoteConfig) -> Result<(), std::io::Error> {
    let config_path = get_config_path();
    let config_content = toml::to_string(config).expect("Failed to serialize configuration");
    fs::write(config_path, config_content)
}
fn get_notes_folder() -> String {
    let config = load_config().expect("Failed to load configuration");
    config.data_location
}
fn display_file_for_current_year(file_name: &str) {
    let notes_folder = get_notes_folder();
    let current_year = Local::now().year();
    let file_path = Path::new(&notes_folder)
        .join(current_year.to_string())
        .join(file_name);
    if file_path.exists() {
        open_file_with_nv(&file_path);
    } else {
        println!("File not found: {}", file_path.display());
    }
}
fn handle_note(note_name: &str, template_name: &str) {
    let file_path = get_note_path(note_name);
    if file_path.exists() {
        open_file_with_nv(&file_path);
    } else {
        create_note_from_template(&file_path, template_name);
        open_file_with_nv(&file_path);
    }
}
fn get_note_path(note_name: &str) -> PathBuf {
    let notes_folder = get_notes_folder();
    let now = Local::now();
    let current_year = now.year();
    let current_month = now.month();
    let current_day = now.day();
    match note_name {
        "day.md" => Path::new(&notes_folder)
            .join(current_year.to_string())
            .join(format!("{:02}", current_month))
            .join(format!("{:02}", current_day))
            .join(note_name),
        "week.md" => Path::new(&notes_folder)
            .join(current_year.to_string())
            .join(format!("{:02}", current_month))
            .join(note_name),
        "month.md" => Path::new(&notes_folder)
            .join(current_year.to_string())
            .join(format!("{:02}", current_month))
            .join(note_name),
        _ => panic!("Unknown note type"),
    }
}
fn create_note_from_template(file_path: &Path, template_name: &str) {
    let notes_folder = get_notes_folder();
    let template_path = Path::new(&notes_folder).join(template_name);
    let template_content = fs::read_to_string(&template_path).expect("Unable to read template");
    fs::create_dir_all(file_path.parent().unwrap()).expect("Unable to create directories");
    fs::write(&file_path, template_content).expect("Unable to write file");
    println!("Created new note from template: {}", file_path.display());
}
fn open_file_with_nv(file_path: &Path) {
    ProcessCommand::new("nvim")
        .arg(file_path)
        .status()
        .expect("Failed to open file with nvim");
}

