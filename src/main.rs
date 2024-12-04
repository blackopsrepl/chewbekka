use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;

use chewbekka::extract::extract_markdown_files_recursive;
use chewbekka::summarize::summarize_content;

#[derive(Parser)]
#[command(
    version = "0.2.3",
    author = "Vittorio Distefano",
    about = "summarizes markdown file(s) at given path"
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
    Summarize(SummarizeOpts),
}

#[derive(Parser)]
struct SummarizeOpts {
    markdown_files: PathBuf,
}

#[tokio::main]
async fn main() {
    let args: Opts = Opts::parse();
    match args.subcmd {
        SubCommand::Summarize(summarize_opts) => {
            subcommand_summarize(summarize_opts).await;
        }
    }
}

async fn subcommand_summarize(summarize_opts: SummarizeOpts) {

    let markdown_files =
    extract_markdown_files_recursive(&summarize_opts.markdown_files).unwrap();

    let mut summarized_files: HashMap<String, String> = HashMap::new();

    for (filename, content) in markdown_files.iter() {
        let summarized_text = summarize_content(content);
        summarized_files.insert(filename.clone(), summarized_text.await);
    }

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
