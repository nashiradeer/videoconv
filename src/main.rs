use std::{
    env::{args, current_dir},
    path::PathBuf,
    process::Command,
};

fn main() {
    let args = args().collect::<Vec<String>>();
    let current_dir = current_dir().unwrap();
    let path = args.get(1).map(PathBuf::from).unwrap_or(current_dir);

    println!("Searching in {:?}", path);

    if path.is_file() {
        if path.extension().unwrap().eq("mkv") {
            convert_file(path);
        }
    } else {
        let entries = path.read_dir().unwrap();
        for entry in entries.flatten() {
            let path = entry.path();

            if let Some(extension) = path.extension() {
                if extension.eq("mkv") {
                    convert_file(path);
                }
            }
        }
    }

    println!("Done");
}

fn convert_file(input: PathBuf) {
    let Some(basename) = input.file_stem().and_then(|s| s.to_str()) else {
        return;
    };

    let output = input.with_file_name(format!("{}.mp4", basename));

    if output.exists() {
        println!("Skipping {:?}", basename);
        return;
    }

    println!("Converting {:?}", basename);

    if let Err(e) = Command::new("ffmpeg")
        .arg("-hwaccel")
        .arg("cuda")
        .arg("-hwaccel_output_format")
        .arg("cuda")
        .arg("-i")
        .arg(&input)
        .arg("-c:v")
        .arg("h264_nvenc")
        .arg("-c:a")
        .arg("aac")
        .arg(output)
        .output()
    {
        eprintln!("Failed to convert {:?}: {:?}", basename, e);
    }
}
