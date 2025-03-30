use clap::Parser;
use std::collections::HashMap;

use std::path::PathBuf;
use std::sync::Mutex;

use chewbekka::extract::extract_markdown_files_recursive;

use chewbekka::process_content;
use chewbekka::write_md_file;

#[derive(Parser)]
#[command(
    version = "1.3.3",
    author = "Vittorio Distefano",
    about = "processes markdown file(s) at given path"
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[clap(
        name = "summarize",
        about = "summarizes markdown file(s) at given path"
    )]
    Summarize(MarkdownFileOpts),

    #[clap(
        name = "expand",
        about = "analyzes all markdown file(s) at a given path as documentation for a task and generates a list of subtasks to be completed"
    )]
    Expand(MarkdownFileOpts),

    #[clap(
        name = "debloat",
        about = "removes unnecessary lingo from markdown file(s) at given path"
    )]
    Debloat(MarkdownFileOpts),
}

#[derive(Parser)]
struct MarkdownFileOpts {
    markdown_files: PathBuf,
}

#[tokio::main]
async fn main() {
    let args: Opts = Opts::parse();
    match args.subcmd {
        SubCommand::Summarize(summarize_opts) => {
            subcommand_summarize(summarize_opts).await;
        }
        SubCommand::Expand(expand_opts) => {
            subcommand_expand(expand_opts).await;
        }
        SubCommand::Debloat(debloat_opts) => {
            subcommand_debloat(debloat_opts).await;
        }
    }
}

async fn subcommand_summarize(summarize_opts: MarkdownFileOpts) {
    let markdown_files = extract_markdown_files_recursive(&summarize_opts.markdown_files).unwrap();

    // let mut summarized_files: HashMap<String, String> = HashMap::new();
    let summarized_files: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

    let markdown_files = markdown_files.lock().unwrap().clone();
    for (filename, content) in markdown_files.iter() {
        let summarized_text = process_content(content, "summarize").await;
        let mut summarized_files = summarized_files.lock().unwrap().clone();
        summarized_files.insert(filename.clone(), summarized_text);
    }

    let summarized_files = summarized_files.lock().unwrap().clone();
    for (filename, summarized_content) in summarized_files.iter() {
        println!(
            "File: {}\nSummarized Content: {}",
            filename, summarized_content
        );
    }

    write_md_file(&summarized_files, true).await;
}

async fn subcommand_expand(expand_opts: MarkdownFileOpts) {
    let markdown_files = extract_markdown_files_recursive(&expand_opts.markdown_files).unwrap();

    let mut expanded_files: HashMap<String, String> = HashMap::new();

    let markdown_files = markdown_files.lock().unwrap().clone();
    for (filename, content) in markdown_files.iter() {
        let expanded_text = process_content(content, "expand").await;
        expanded_files.insert(filename.clone(), expanded_text);
    }

    write_md_file(&expanded_files, true).await;
}

async fn subcommand_debloat(debloat_opts: MarkdownFileOpts) {
    let markdown_files = extract_markdown_files_recursive(&debloat_opts.markdown_files).unwrap();

    let debloated_files: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

    let markdown_files = markdown_files.lock().unwrap().clone();
    for (filename, content) in markdown_files.iter() {
        let nojargon_text = process_content(content, "debloat").await;
        let mut debloated_files = debloated_files.lock().unwrap();
        debloated_files.insert(filename.clone(), nojargon_text);
    }

    let debloated_files = debloated_files.lock().unwrap().clone();
    for (filename, debloated_content) in debloated_files.iter() {
        println!(
            "File: {}\nDebloated Content: {}",
            filename, debloated_content
        );
    }

    write_md_file(&debloated_files, false).await;
}
