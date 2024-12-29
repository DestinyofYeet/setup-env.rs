use std::{env, fs, path::{Path, PathBuf}, process::exit, str::FromStr};

use clap::Parser;


#[derive(Parser, Debug)]
#[command(version = "1.0", about = "A tool to generate a working environment", long_about = None)]
struct Args {
    #[arg(short = 'u', long = "update", default_value = "false", help = "Check for updates")]
    update: bool,

    #[arg(short = 'l', long = "lang", required = true, help = "The language to setup")]
    language: String,

    #[arg(short = 'd', long = "directory", help = "The directory to init the environment in", default_value = ".")]
    directory: String,

    #[arg(short = None, long = "envs", help = "The directory with the environment folders")]
    env_dir: Option<String>,
}

struct Data {
    args: Args,

    bin_dir: Option<String>,
    env_dir: Option<String>,
}

fn copy_folder_dir(src: impl AsRef<Path>, dest: impl AsRef<Path>, overwrite: bool) {

    if !fs::exists(&dest).expect(&format!("Failed to check if '{}' exists", dest.as_ref().to_str().unwrap())) {
        match fs::create_dir(&dest) {
            Ok(_) => {},
            Err(_) => panic!("Could not create dst folder '{}'", dest.as_ref().to_str().unwrap()),
        }
    } else {
        println!("'{}' exists", dest.as_ref().to_str().unwrap());
    }
    

    match fs::read_dir(&src) {
        Err(_) => panic!("Cold not read src folder '{}'", src.as_ref().to_str().unwrap()),
        Ok(handle) => {
            for entry in handle {
                let entry = entry.unwrap();
                let file_type = entry.file_type().unwrap();

                if file_type.is_dir() {
                    copy_folder_dir(entry.path(), dest.as_ref().join(entry.file_name()), overwrite);
                } else {
                    let new_file_path = dest.as_ref().join(entry.file_name());
                    if !fs::exists(&dest).expect(&format!("Failed to check if '{}' exists", dest.as_ref().to_str().unwrap())) || overwrite {
                        
                        match fs::copy(entry.path(), &new_file_path) {
                            Ok(_) => { println!("Copied '{}' to '{}'", entry.path().to_str().unwrap(), &new_file_path.to_str().unwrap()) },
                            Err(e) => eprintln!("Failed to copy file {} {}", entry.file_name().to_str().unwrap(), e)
                        }
                    } else {
                        println!("'{}' exists, not overwriting", &new_file_path.to_str().unwrap());
                    }
                }
            }
        }
    }
}

fn env_setup_rust(data: &Data){
    println!("Setting up rust at '{}'", data.args.directory);

    copy_folder_dir(&format!("{}/rust", data.env_dir.as_ref().unwrap()), &data.args.directory, false);
}

fn convert_string_to_path(path: impl AsRef<Path>) -> impl AsRef<Path> {
    return path;
}

fn main() {
    let args = Args::parse();

    let mut data = Data {
        args,
        bin_dir: None,
        env_dir: None,
    };

    match env::current_exe() {
        Ok(exe_path) => data.bin_dir = Some(exe_path.display().to_string()),

        Err(e) => panic!("Failed to get current bin path: {e}"),
    }

    data.env_dir = match &data.args.env_dir {
        None => {
            Some(String::from_str(convert_string_to_path(data.bin_dir.as_ref().unwrap()).as_ref().parent().unwrap().parent().unwrap().join("envs").to_str().unwrap()).unwrap())
        },
        Some(value) => Some(format!("{}", value)),
    };


    println!("Bin is in {}", data.bin_dir.as_ref().unwrap());

    match data.args.language.as_str() {
        "rust" => {
            env_setup_rust(&data);
        },
        
        _ => {
            eprintln!("Error: Language '{}' is not supported", data.args.language);
            exit(1)
        }
    }
}
