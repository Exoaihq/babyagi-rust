use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    let repository_path = Path::new("playground");
    let workspace_path = Path::new("playground_data");

    // Create the workspace directory if it doesn't exist
    if !workspace_path.exists() {
        fs::create_dir_all(workspace_path).unwrap();
    }

    // Extract and save info to CSV
    let info = extract_info(repository_path);
    save_info_to_csv(&info, workspace_path.join("repository_info.csv"));

    // TODO: Add the rest of the logic for computing and saving embeddings
}

fn extract_info(repository_path: &Path) -> Vec<(String, (usize, usize), String)> {
    let mut info = Vec::new();
    let lines_per_chunk = 60;

    for entry in fs::read_dir(repository_path).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();

        if file_path.is_file() {
            let file = File::open(&file_path).unwrap();
            let reader = BufReader::new(file);
            let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

            let chunks = lines.chunks(lines_per_chunk);

            for (i, chunk) in chunks.enumerate() {
                let chunk_content = chunk.join("\n");
                let first_line = i * lines_per_chunk + 1;
                let last_line = first_line + chunk.len() - 1;
                let line_coverage = (first_line, last_line);

                info.push((file_path.to_string_lossy().to_string(), line_coverage, chunk_content));
            }
        }
    }

    info
}

fn save_info_to_csv(info: &[(String, (usize, usize), String)], csv_path: impl AsRef<Path>) {
    let mut csv_file = File::create(csv_path).unwrap();
    writeln!(csv_file, "filePath,lineCoverage,content").unwrap();

    for (file_path, (first_line, last_line), content) in info {
        let line_coverage = format!("{}-{}", first_line, last_line);
        let content = content.replace("\n", "\\n").replace(",", "\\,");
        writeln!(csv_file, "{},{},{}", file_path, line_coverage, content).unwrap();
    }
}