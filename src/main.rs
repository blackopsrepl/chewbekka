
use clap::Parser;
use std::collections::HashMap;

use std::path::PathBuf;
use std::sync::Mutex;

use chewbekka::extract::extract_markdown_files_recursive;
use chewbekka::summarize::summarize_content;
use chewbekka::expand::expand;
use chewbekka::debloat::dissect_subtlety;

#[derive(Parser)]
#[command(
    version = "1.2.0",
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
        },
        SubCommand::Expand(expand_opts) => {
            subcommand_expand(expand_opts).await;
        },
        SubCommand::Debloat(debloat_opts) => {
            subcommand_debloat(debloat_opts).await;
        }
    }
}

async fn subcommand_summarize(summarize_opts: MarkdownFileOpts) {

    let markdown_files =
    extract_markdown_files_recursive(&summarize_opts.markdown_files).unwrap();

    // let mut summarized_files: HashMap<String, String> = HashMap::new();
    let summarized_files: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

    let markdown_files = markdown_files.lock().unwrap();
    for (filename, content) in markdown_files.iter() {
        let summarized_text = summarize_content(content);
        let mut summarized_files = summarized_files.lock().unwrap();
        summarized_files.insert(filename.clone(), summarized_text.await);
    }

    let summarized_files = summarized_files.lock().unwrap();
    for (filename, summarized_content) in summarized_files.iter() {
        println!(
            "File: {}\nSummarized Content: {}",
            filename, summarized_content
        );
    }

    let concatenated_summary: String = summarized_files
    .values()
    .cloned()
    .collect::<Vec<String>>()
    .join("\n\n");

    // summarize and write to an md file
    let output_file = "output.md";
    let output = summarize_content(&concatenated_summary).await;

    // write summary to file
    std::fs::write(output_file, output).unwrap();
}

async fn subcommand_expand(expand_opts: MarkdownFileOpts) {
    let markdown_files =
    extract_markdown_files_recursive(&expand_opts.markdown_files).unwrap();

    let mut expanded_files: HashMap<String, String> = HashMap::new();

    let markdown_files = markdown_files.lock().unwrap();
    for (filename, content) in markdown_files.iter() {
        let expanded_text = expand(content).await;
        expanded_files.insert(filename.clone(), expanded_text);
    }

    let concatenated_expanded: String = expanded_files
    .values()
    .cloned()
    .collect::<Vec<String>>()
    .join("\n\n");

    // summarize and write to an md file
    let output_file = "output.md";
    let output = summarize_content(&concatenated_expanded).await;

    // write summary to file
    std::fs::write(output_file, output).unwrap();

}

async fn subcommand_debloat(debloat_opts: MarkdownFileOpts) {
    // first dissect subtlety, then strip jargon, then summarize all
        let markdown_files =
        extract_markdown_files_recursive(&debloat_opts.markdown_files).unwrap();
    
        let debloated_files: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    
        let markdown_files = markdown_files.lock().unwrap();
        for (filename, content) in markdown_files.iter() {
            let nojargon_text = dissect_subtlety(content);
            let mut debloated_files: std::sync::MutexGuard<'_, HashMap<String, String>> = debloated_files.lock().unwrap();
            debloated_files.insert(filename.clone(), nojargon_text.await);
        }
    
        let debloated_files = debloated_files.lock().unwrap();
        for (filename, debloated_content) in debloated_files.iter() {
            println!(
                "File: {}\nDebloated Content: {}",
                filename, debloated_content
            );
        }
    
        let concatenated_debloated: String = debloated_files
        .values()
        .cloned()
        .collect::<Vec<String>>()
        .join("\n\n");
    
        // summarize and write to an md file
        let output_file = "output.md";
        let output = &concatenated_debloated;
    
        // write summary to file
        std::fs::write(output_file, output).unwrap();
    }
                                                                                                                                                                      