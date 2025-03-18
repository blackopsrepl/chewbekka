use clap::Parser;
use std::collections::HashMap;

use std::path::PathBuf;
use std::sync::Mutex;

use chewbekka::extract::extract_markdown_files_recursive;
use chewbekka::summarize::summarize_content;
use chewbekka::debloat::dissect_subtlety;
use chewbekka::debloat::strip_jargon;

#[derive(Parser)]
#[command(
    version = "0.3.0",
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
    let output = &concatenated_summary;

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
        let nojargon_text = strip_jargon(content);
        let debloated_text = dissect_subtlety(&nojargon_text.await).await;
        let mut debloated_files: std::sync::MutexGuard<'_, HashMap<String, String>> = debloated_files.lock().unwrap();
        debloated_files.insert(filename.clone(), debloated_text);
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
    let output = summarize_content(&concatenated_debloated).await;

    // write summary to file
    std::fs::write(output_file, output).unwrap();
}
                                                                                                                                                                      