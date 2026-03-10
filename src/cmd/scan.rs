use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, u16};
use std::io::{BufRead, BufReader};
use super::command_prelude::*;
use crate::{get_book_dir};
use anyhow::Result;
use mdbook_core::config::{Config};
use tracing::{debug, trace};

const FOLDER_MD_FILE: &str = "README.md";
const SUMMARY_MD_FILE: &str = "SUMMARY.md";
const SCAN_FILE_EXT: &str = "md";
const MAX_READ_LINES: usize = 10;
const IDENT_STR: &str = "  ";
const CHAPTER_TITLE_PREFIX: &str = "# ";
const CHAPTER_COMMENT_PREFIX: &str = "<!--";
const CHAPTER_COMMENT_SUFFIX: &str = "-->";
const COMMENT_PARSER_TITLE_PREFIX: &str = "title=";
const COMMENT_PARSER_ORDER_PREFIX: &str = "order=";

#[derive(Debug)]
struct ChapterInfo {
    title: Option<String>,
    order: Option<u16>,
}

#[derive(Debug)]
enum CommentValue {
    Title(String),
    Order(u16),
    None,
}

#[derive(Debug)]
enum TreeItem {
    File(PathBuf, String),
    Dir(PathBuf, String),
}

fn parse_comment_content(s: String) -> CommentValue {
    if s.starts_with(COMMENT_PARSER_TITLE_PREFIX) {
        CommentValue::Title(s[6..].to_string())
    } else if s.starts_with(COMMENT_PARSER_ORDER_PREFIX) {
        match s[6..].parse::<u16>() {
            Ok(n) => CommentValue::Order(n),
            Err(_) => CommentValue::None,
        }
    } else {
        CommentValue::None
    }
}

fn read_info_from_file(file_path: &Path) -> ChapterInfo {
    let mut ch_info =  ChapterInfo{
        title: None,
        order: None,
    };

    if let Ok(f) = fs::File::open(file_path) {
        let reader = BufReader::new(f);
        for (idx, line) in reader.lines().take(MAX_READ_LINES).enumerate() {
            let v = line.unwrap();
            let trimmed = v.trim();
            if trimmed.len() == 0 {
                continue;
            }

            if idx == 0 && trimmed.starts_with(CHAPTER_TITLE_PREFIX) {
                let content = &trimmed[2..];
                ch_info.title = Some(content.trim().to_string());
            }

            if trimmed.starts_with(CHAPTER_COMMENT_PREFIX) && trimmed.ends_with(CHAPTER_COMMENT_SUFFIX) {
                let content = trimmed[4..trimmed.len()-3].trim();
                let comment_val = parse_comment_content(content.to_string());
                match comment_val {
                    CommentValue::Title(t) => ch_info.title = Some(t),
                    CommentValue::Order(o) => ch_info.order = Some(o),
                    _ => (),
                }
            }
        }
    }


    ch_info
}


fn get_info(path: &Path) -> ChapterInfo {
    if path.is_dir() {
        let readme = path.join(FOLDER_MD_FILE);
        read_info_from_file(&readme)
    } else {
        read_info_from_file(path)
    }
}

// fn print_subtree(path: &Path) {
//     println!("{}", gen_summary(path));
// }

fn get_summary_line(depth: usize, name: String, path: String) -> String {
    format!("{}- [{}]({})\n", IDENT_STR.repeat(depth - 1), name, path)
}

fn gen_summary(path: &Path) -> String {
    let mut stack = Vec::new();
    stack.push((TreeItem::Dir(path.to_path_buf(), String::new()), 0));
    let mut tree_str = String::new();
    while let Some((item, current_depth)) = stack.pop() {
        match item {
            TreeItem::File(p, n) => {
                let rel = p.strip_prefix(path).unwrap().display().to_string();
                tree_str.push_str(&get_summary_line(current_depth,n, rel));
            }
            TreeItem::Dir(p,n) => {
                if current_depth > 0 {
                    let sub_readme = p.join(FOLDER_MD_FILE);
                    let rel = if sub_readme.is_file() {
                        sub_readme.strip_prefix(path).unwrap().display().to_string()
                    } else {
                        String::new()
                    };
                    // let rel = p.strip_prefix(path).unwrap();
                    tree_str.push_str(&get_summary_line(current_depth, n,rel));
                }

                let entries = fs::read_dir(&p).unwrap();

                let mut items: Vec<_> = entries.filter_map(|entry| {
                    let entry = entry.ok()?;
                    let path = entry.path();

                    if path.is_file() {
                        if let Some(ext) = path.extension() && ext == SCAN_FILE_EXT {
                            let filename = path.file_name().unwrap().display().to_string();
                            // skip top summary.md
                            if current_depth == 0 && filename == SUMMARY_MD_FILE {
                                return None;
                            }

                            // skip subpath readme.md
                            if current_depth > 0 && filename == FOLDER_MD_FILE {
                                return None;
                            }

                            return Some(path);
                        }
                    } else if path.is_dir() {
                        return Some(path);
                    }
                    None
                }).collect();

                let mut title_map: HashMap<String, String> = HashMap::new();
                items.sort_by_key(|path| {
                    let info: ChapterInfo = get_info(&path);
                    if let Some(title) = info.title {
                        title_map.insert(path.display().to_string(), title);
                    }

                    info.order.unwrap_or(u16::MAX)
                });

                for path in items.into_iter().rev() {
                    let fullpath = path.display().to_string();
                    let name = if let Some(k) = title_map.get(&fullpath) {
                        k.to_string()
                    } else {
                        let filename = path.file_name().unwrap().display().to_string();
                        if let Some(e) = path.extension() && e == SCAN_FILE_EXT {
                            filename[..filename.len()-3].to_string()
                        } else {
                            filename
                        }

                    };

                    let item =  if path.is_dir() {
                        TreeItem::Dir(path, name)
                    } else {
                        TreeItem::File(path, name)
                    };

                    stack.push((item, current_depth + 1));
                }
            }
        }
    }

    tree_str
}

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("scan")
        .about("Scan src files and gen SUMMARY.md")
        .arg_dest_dir()
        .arg_root_dir()
        .arg_open()
}

// Build command implementation
pub fn execute(args: &ArgMatches) -> Result<()> {
    let book_dir = get_book_dir(args);
    let config = load_config(&book_dir)?;
    let src_dir = book_dir.join(&config.book.src);
    // print_subtree(&src_dir);
    fs::write(src_dir.join(SUMMARY_MD_FILE), gen_summary(&src_dir))?;

    Ok(())
}

pub fn load_config<P: Into<PathBuf>>(book_root: P) -> Result<Config> {
    let book_root = book_root.into();
    let config_location = book_root.join("book.toml");

    let mut config = if config_location.exists() {
        debug!("Loading config from {}", config_location.display());
        Config::from_disk(&config_location)?
    } else {
        Config::default()
    };

    config.update_from_env()?;

    if tracing::enabled!(tracing::Level::TRACE) {
        for line in format!("Config: {config:#?}").lines() {
            trace!("{}", line);
        }
    }

    Ok(config)
}