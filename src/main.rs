use std::{env, fs::{self, Permissions}, os::unix::fs::PermissionsExt, path::Path, process::exit, str::FromStr};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "A tool to generate a working environment in NixOS", long_about = None)]
struct Args {
    #[arg(
        short = 'u',
        long = "update",
        default_value = "false",
        help = "Check for updates"
    )]
    update: bool,

    #[arg(
        required = true,
        help = "The language to setup",
        index = 1,
    )]
    language: String,

    #[arg(
        short = 'd',
        long = "directory",
        help = "The directory to init the environment in",
        default_value = "."
    )]
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
    if !fs::exists(&dest).expect(&format!(
        "Failed to check if '{}' exists",
        dest.as_ref().to_str().unwrap()
    )) {
        match fs::create_dir(&dest) {
            Ok(_) => {
                // TODO: set permissions for subfolders too
                // folder.metadata().unwrap().permissions().set_mode(0o754)
            }
            Err(_) => panic!(
                "Could not create dst folder '{}'",
                dest.as_ref().to_str().unwrap()
            ),
        }
    } else {
        println!("'{}' exists", dest.as_ref().to_str().unwrap());
    }

    match fs::read_dir(&src) {
        Err(_) => panic!(
            "Cold not read src folder '{}'",
            src.as_ref().to_str().unwrap()
        ),
        Ok(handle) => {
            for entry in handle {
                let entry = entry.unwrap();
                let file_type = entry.file_type().unwrap();

                if file_type.is_dir() {
                    copy_folder_dir(
                        entry.path(),
                        dest.as_ref().join(entry.file_name()),
                        overwrite,
                    );
                } else {
                    let new_file_path = dest.as_ref().join(entry.file_name());

                    if !(fs::exists(&new_file_path).expect(&format!(
                        "Failed to check if '{}' exists",
                        dest.as_ref().to_str().unwrap()
                    ))) || overwrite
                    {
                        match fs::copy(entry.path(), &new_file_path) {
                            Ok(_) => {
                                println!(
                                    "Copied '{}' to '{}'",
                                    entry.path().to_str().unwrap(),
                                    &new_file_path.to_str().unwrap()
                                );

                                match fs::metadata(&new_file_path.to_str().unwrap()) {
                                    Err(e) => { eprintln!("Failed to get metadata for file '{}' because: {}", &new_file_path.to_str().unwrap(), e)},
                                    Ok(metadata) => {
                                        let mut permissions = metadata.permissions();
                                        permissions.set_mode(0o644);
                                        match fs::set_permissions(&new_file_path, permissions) {
                                            Err(e) => {
                                                eprintln!("Failed to make file '{}' writeable because: {}", &new_file_path.to_str().unwrap(), e)
                                            }
                                            Ok(_) => {}
                                        }
                                    }
                                }
                            }
                            Err(e) => eprintln!(
                                "Failed to copy file {} {}",
                                entry.file_name().to_str().unwrap(),
                                e
                            ),
                        }
                    } else {
                        println!(
                            "'{}' exists, not overwriting",
                            &new_file_path.to_str().unwrap()
                        );
                    }
                }
            }
        }
    }
}

fn env_setup_generic(data: &Data, env: &str) {
    println!("Setting up {} at '{}'", env, data.args.directory);

    copy_folder_dir(
        &format!("{}/{}", data.env_dir.as_ref().unwrap(), env),
        &data.args.directory,
        false,
    );
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
        None => Some(
            String::from_str(
                convert_string_to_path(data.bin_dir.as_ref().unwrap())
                    .as_ref()
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("envs")
                    .to_str()
                    .unwrap(),
            )
            .unwrap(),
        ),
        Some(value) => Some(format!("{}", value)),
    };

    println!("Bin is in {}", data.bin_dir.as_ref().unwrap());

    match data.args.language.as_str() {
        "rust" => {
            env_setup_generic(&data, "rust");
        }

        "python" => {
            env_setup_generic(&data, "python");
        }

        "d2" => {
            env_setup_generic(&data, "d2");
        }

        "hugo" => {
            env_setup_generic(&data, "hugo");
        }

        _ => {
            eprintln!("Error: Language '{}' is not supported", data.args.language);
            exit(1)
        }
    }
}
