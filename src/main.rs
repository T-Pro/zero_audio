use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    println!("Starting to confuse the audio...");

    let paths = std::fs::read_dir("./input")?;

    for path in paths {

        let path: std::path::PathBuf = path.unwrap().path();
        let path_c = path.clone();
        let path_str = path_c.display();
        println!("Processing: {}", path_str);

        let file_len = std::fs::metadata(&path).unwrap().len();
        let file_len_no_header = file_len - 4096;

        let output = format!("./output/{}", path.file_name().unwrap().to_str().unwrap());

        let mut input = File::open(path).unwrap();
        let mut output = File::create(output).unwrap();

        const BUFFER_SIZE: usize = 4096;
        let mut buffer = [0; BUFFER_SIZE];

        let read = input.read(&mut buffer).unwrap();

        output.write(&buffer[..read]);

        for i in 0..file_len_no_header {
            output.write(&[0]);
        }

        println!("{} processed.", path_str);

    }
    
    Ok(())
}
