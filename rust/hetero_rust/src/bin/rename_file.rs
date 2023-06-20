use std::fs;

fn main() -> std::io::Result<()> {
    let path = "/home/sjq/git/hetero_py/splited_graph"; // replace with the actual folder you want to rename files in
    let graphs = ["dblp", "imdb", "last-fm", "ogb-mag"];
    for graph in graphs.iter() {
        let path = format!("{}/{}", path, graph);
        let entries = fs::read_dir(path)?; // read directory
        for entry in entries {
            let entry = entry?; // unwrap the entry Result
            let file_name = entry.file_name(); // get the file name
            let new_file_name = file_name.to_string_lossy().replace("_", "$"); // replace "_" with "$" in the file name
            let new_path = entry.path().with_file_name(new_file_name); // create the new path with the new file name
            fs::rename(entry.path(), new_path)?; // rename the file
        }
    }

    Ok(())
}
