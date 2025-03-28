pub mod async_wrapper;
pub mod debloat;
pub mod expand;
pub mod extract;
pub mod summarize;

use std::collections::HashMap;
use summarize::summarize_content;

pub async fn write_md_file(output_files: &HashMap<String, String>, summmarize: bool) {
    let concatenated_output: String = output_files
        .values()
        .cloned()
        .collect::<Vec<String>>()
        .join("\n\n");

    let output_file = "output.md";
    if summmarize {
        let output = summarize_content(&concatenated_output).await;
        std::fs::write(output_file, output).unwrap();
    } else {
        std::fs::write(output_file, &concatenated_output).unwrap();
    }
}
