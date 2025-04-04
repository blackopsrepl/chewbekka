use clap::Parser;
use std::collections::HashMap;

use std::path::PathBuf;
use std::sync::Mutex;

use chewbekka::extract::extract_files_recursive;

use chewbekka::process_content;
use chewbekka::write_md_file;
use chewbekka::pre_tasks;
use chewbekka::post_tasks;

#[derive(Parser)]
#[command(
    version = "1.5.0",
    author = "Vittorio Distefano",
    about = "processes text file(s) at given path"
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[clap(name = "summarize", about = "summarizes text file(s) at given path")]
    Summarize(SubcommandOpts),

    #[clap(
        name = "expand",
        about = "analyzes all text file(s) at a given path as documentation for a task and generates a list of subtasks to be completed"
    )]
    Expand(SubcommandOpts),

    #[clap(
        name = "debloat",
        about = "removes unnecessary lingo from text file(s) at given path"
    )]
    Debloat(SubcommandOpts),

    #[clap(name = "docugen", about = "generates documentation for a codebase")]
    Docugen(SubcommandOpts),
}

#[derive(Parser)]
struct SubcommandOpts {
    markdown_files: PathBuf,
    #[clap(long, help = "output markdown file path")]
    output_markdown: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args: Opts = Opts::parse();
    match args.subcmd {
        SubCommand::Summarize(summarize_opts) => {
            subcommand_handler(summarize_opts, "summarize", &vec!["md", "txt"]).await;
        }
        SubCommand::Expand(expand_opts) => {
            subcommand_handler(expand_opts, "expand", &vec!["md", "txt"]).await;
        }
        SubCommand::Debloat(debloat_opts) => {
            subcommand_handler(debloat_opts, "debloat", &vec!["md", "txt"]).await;
        }
        SubCommand::Docugen(docugen_opts) => {
            subcommand_handler(docugen_opts, "docugen", &vec!["rs"]).await;
        }
    }
}

async fn subcommand_handler(subcommand_opts: SubcommandOpts, task: &str, extensions: &Vec<&str>) {
    let input_files = extract_files_recursive(&subcommand_opts.markdown_files, extensions).unwrap();
    let processed_files = Mutex::new(HashMap::new());

    // pre-tasks
    let input_files = pre_tasks(input_files, task).await.lock().unwrap().clone();

    // process contents
    for (filename, content) in input_files.iter() {
        let processed_text = process_content(content, task).await;
        let mut processed_files = processed_files.lock().unwrap();
        processed_files.insert(filename.clone(), processed_text);
    }

    // post-tasks
    let processed_files = post_tasks(processed_files, task).await.lock().unwrap().clone();

    // console
    for (filename, processed_content) in processed_files.iter() {
        println!(
            "File: {}\n\nProcessed Content: {}\n\n",
            filename, processed_content
        );
    }

    // write to md
    if subcommand_opts.output_markdown.is_some() {
        write_md_file(&processed_files, &subcommand_opts.output_markdown.unwrap()).await;
    }
}
