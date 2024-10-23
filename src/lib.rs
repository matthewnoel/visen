use std::env;

use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

#[derive(Debug)]
pub enum ScriptError {
    Io(std::io::Error),
}

impl From<std::io::Error> for ScriptError {
    fn from(err: std::io::Error) -> Self {
        ScriptError::Io(err)
    }
}

pub struct Script {
    pub title: String,
    pub text: String,
    pub word_count: u64,
    pub dialogue_word_count: u64,
    pub blocked_seconds: u64,
}

impl std::fmt::Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\n\"{}\"\nEstimated runtime: {}\nEstimated dialogue time: {}\nWord count (dialogue): {}\nWord count (total): {}\n",
            self.title,
            seconds_to_human_shorthand_duration(self.blocked_seconds + word_count_to_seconds(self.dialogue_word_count)),
            seconds_to_human_shorthand_duration(word_count_to_seconds(self.dialogue_word_count)),
            self.dialogue_word_count,
            self.word_count
        )
    }
}

pub fn init(project_name: &String) -> Result<(), ScriptError> {
    let path = std::path::Path::new(project_name);
    if path.exists() {
        return Err(ScriptError::Io(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Project directory already exists",
        )));
    }
    std::fs::create_dir_all(path)?;
    env::set_current_dir(path)?;
    std::fs::write(".visenrc", "v0.1.0\n")?;
    std::fs::write("SCRIPT.md", format!("# {}\n\n", project_name))?;
    return Ok(());
}

pub fn validate_command_is_running_inside_visen_project() -> Result<(), ScriptError> {
    if std::path::Path::new(".visenrc").exists() {
        return Ok(());
    }
    return Err(ScriptError::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        ".visenrc file not found. Are you sure you're in a visen project?",
    )));
}

pub fn build_script() -> Result<Script, ScriptError> {
    let text = std::fs::read_to_string("SCRIPT.md")?;
    let parser = Parser::new(&text);
    let mut title = String::new();
    let mut word_count = 0;
    let mut blocked_seconds = 0;
    let mut is_in_block_quote = false;
    let mut block_quote_text = String::new();
    let mut is_in_h1 = false;
    let mut is_in_code_block = false;
    let mut dialogue_word_count: u64 = 0;
    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::BlockQuote(_) => is_in_block_quote = true,
                Tag::Heading {
                    level,
                    id: _,
                    classes: _,
                    attrs: _,
                } => match level {
                    HeadingLevel::H1 => is_in_h1 = true,
                    _ => (),
                },
                Tag::CodeBlock(_) => is_in_code_block = true,
                _ => (),
            },
            Event::End(tag_end) => match tag_end {
                TagEnd::BlockQuote(_) => {
                    match block_quote_text.parse::<humantime::Duration>() {
                        Ok(x) => blocked_seconds += x.as_secs(),
                        Err(_) => (),
                    };
                    block_quote_text.clear();
                    is_in_block_quote = false;
                }
                TagEnd::Heading(HeadingLevel::H1) => is_in_h1 = false,
                TagEnd::CodeBlock => is_in_code_block = false,
                _ => (),
            },
            Event::Text(text) => {
                word_count += text.split_whitespace().count() as u64;
                if is_in_block_quote {
                    block_quote_text.push_str(&text);
                }
                if is_in_h1 {
                    title.push_str(&text);
                }
                if is_in_code_block {
                    dialogue_word_count += text
                        .split_whitespace()
                        .filter(|word| {
                            let is_fully_capitalized = word.chars().all(|c| c.is_uppercase());
                            !is_fully_capitalized
                        })
                        .count() as u64;
                }
            }
            _ => (),
        }
    }

    return Ok(Script {
        title,
        text,
        word_count,
        dialogue_word_count,
        blocked_seconds,
    });
}

pub fn write_html(script: &Script) -> Result<(), ScriptError> {
    let parser = Parser::new(&script.text);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    std::fs::create_dir_all("./docs")?;
    std::fs::write("./docs/index.html", html_output)?;
    return Ok(());
}

pub fn write_readme(script: &Script) -> Result<(), ScriptError> {
    let readme = format!(
        "# {}\n\nEstimated runtime: {}\n\nWord count (dialogue): {}\n\nWord count (total): {}\n",
        script.title,
        seconds_to_human_shorthand_duration(script.blocked_seconds + word_count_to_seconds(script.dialogue_word_count)),
        script.dialogue_word_count,
        script.word_count
    );
    std::fs::write("./README.md", readme)?;
    return Ok(());
}

fn seconds_to_human_shorthand_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    let mut formatted_time = String::new();

    if hours > 0 {
        formatted_time.push_str(&format!("{}h", hours));
    }

    if minutes > 0 {
        if formatted_time.len() > 0 {
            formatted_time.push_str(" ");
        }
        formatted_time.push_str(&format!("{}m", minutes));
    }

    if seconds > 0 {
        if formatted_time.len() > 0 {
            formatted_time.push_str(" ");
        }
        formatted_time.push_str(&format!("{}s", seconds));
    }

    return formatted_time;
}

fn word_count_to_seconds(word_count: u64) -> u64 {
    return (word_count as f64 * 2.5).round() as u64;
}
