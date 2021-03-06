use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    println!("Starting to confuse the audio...");

    let raw_paths: std::fs::ReadDir = std::fs::read_dir("./input")?;

    let mut filtered_paths = raw_paths.filter_map(|entry| {
        let path: std::path::PathBuf = entry.ok()?.path();
        match path.extension().or(None)?.to_str().unwrap() {
            "caf" | "caff" | "wav" | "pcm" | "raw" => Some(path),
            _ => None,
        }
    });

    let paths = filtered_paths.collect::<Vec<std::path::PathBuf>>();

    if paths.is_empty() {
        println!("No input caf, caff, wav, pcm or raw files found");
        return Ok(())
    }

    for path in paths {

        let path_c = path.clone();
        let path_str = path_c.display();
        println!("Processing: {}", path_str);

        let header_size = match path_c.extension().unwrap().to_str().unwrap() {
            "caf" | "caff" => 4096,
            "wav" => 44,
            _ => 0,
        };

        let file_len = std::fs::metadata(&path).unwrap().len();
        let file_len_no_header = file_len - header_size;

        let output = format!("./output/{}", path.file_name().unwrap().to_str().unwrap());

        let mut input = File::open(path).unwrap();
        let mut output = File::create(output).unwrap();

        const BUFFER_SIZE: usize = 4096;
        let mut buffer = [0; BUFFER_SIZE];

        let read = input.read(&mut buffer).unwrap();

        output.write(&buffer[..read]);

        for _ in 0..file_len_no_header {
            output.write(&[0]);
        }

        println!("{} processed.", path_str);

    }
    
    Ok(())
}
