use clap::Parser;
use std::collections::HashMap;

use std::path::PathBuf;
use std::sync::Mutex;

use chewbekka::extract::extract_files_recursive;

use chewbekka::process_content;
use chewbekka::write_md_file;

#[derive(Parser)]
#[command(
    version = "1.4.0",
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
    Summarize(SubcommandOpts),

    #[clap(
        name = "expand",
        about = "analyzes all markdown file(s) at a given path as documentation for a task and generates a list of subtasks to be completed"
    )]
    Expand(SubcommandOpts),

    #[clap(
        name = "debloat",
        about = "removes unnecessary lingo from markdown file(s) at given path"
    )]
    Debloat(SubcommandOpts),
}

#[derive(Parser)]
struct SubcommandOpts {
    markdown_files: PathBuf,
    #[clap(long, required = false, help = "output markdown file path")]
    output_markdown: PathBuf,
}

#[tokio::main]
async fn main() {
    let args: Opts = Opts::parse();
    match args.subcmd {
        SubCommand::Summarize(summarize_opts) => {
            subcommand_handler(summarize_opts, "summarize", &vec!["md", "txt"], true).await;
        }
        SubCommand::Expand(expand_opts) => {
            subcommand_handler(expand_opts, "expand", &vec!["md", "txt"], false).await;
        }
        SubCommand::Debloat(debloat_opts) => {
            subcommand_handler(debloat_opts, "debloat", &vec!["md", "txt"], false).await;
        }
    }
}

async fn subcommand_handler(subcommand_opts: SubcommandOpts, task: &str, extensions: &Vec<&str>, summarize: bool) {
    let input_files = extract_files_recursive(&subcommand_opts.markdown_files, extensions).unwrap();

    let processed_files = Mutex::new(HashMap::new());

    let input_files = input_files.lock().unwrap().clone();
    
    for (filename, content) in input_files.iter() {
        let processed_text = process_content(content, task).await;
        let mut processed_files = processed_files.lock().unwrap();
        processed_files.insert(filename.clone(), processed_text);
    }

    let processed_files = processed_files.lock().unwrap().clone();
    for (filename, processed_content) in processed_files.iter() {
        println!(
            "File: {}\n\nProcessed Content: {}\n\n",
            filename, processed_content
        );
    }

    if subcommand_opts.output_markdown.exists() {
        write_md_file(
            &processed_files,
            &subcommand_opts.output_markdown,
            // TODO: refactor summarization to post_tasks system
            summarize,
        )
        .await;
    }
}
