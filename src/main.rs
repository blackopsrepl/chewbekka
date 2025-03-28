use clap::Parser;
use std::collections::HashMap;

use std::path::PathBuf;
use std::sync::Mutex;

use chewbekka::debloat::debloat;
use chewbekka::expand::expand;
use chewbekka::extract::extract_markdown_files_recursive;
use chewbekka::summarize::summarize_content;
use chewbekka::write_md_file;

#[derive(Parser)]
#[command(
    version = "1.3.1",
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

    let summarized_files: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

    let markdown_files = markdown_files.lock().unwrap().clone();
    for (filename, content) in markdown_files.iter() {
        let summarized_text = summarize_content(content);
        let mut summarized_files = summarized_files.lock().unwrap().clone();
        summarized_files.insert(filename.clone(), summarized_text.await);
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

    let expanded_files: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

    let markdown_files = markdown_files.lock().unwrap().clone();
    for (filename, content) in markdown_files.iter() {
        let expanded_text = expand(content);
        let mut expanded_files = expanded_files.lock().unwrap().clone();
        expanded_files.insert(filename.clone(), expanded_text.await);
    }

    let expanded_files = expanded_files.lock().unwrap().clone();
    for (filename, expanded_content) in expanded_files.iter() {
        println!("File: {}Expanded Content: {}", filename, expanded_content);
    }

    write_md_file(&expanded_files, true).await;
}

async fn subcommand_debloat(debloat_opts: MarkdownFileOpts) {
    let markdown_files = extract_markdown_files_recursive(&debloat_opts.markdown_files).unwrap();

    let debloated_files: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

    let markdown_files = markdown_files.lock().unwrap().clone();
    for (filename, content) in markdown_files.iter() {
        let nojargon_text = debloat(content).await;
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
