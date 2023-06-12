fn main() {
    let path = "file_to_delete.txt";
    std::fs::remove_file(path).expect("Failed to delete the file");
}