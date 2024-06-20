mod sources;

use std::error::Error;
use std::fs::{create_dir, create_dir_all, write};
use std::path::Path;
use std::env::args;
use std::time::Instant;

use crate::sources::Sources;

const VERSION: &str = "0.1.0";
const COLOR_RED_BOLD: &str = "\x1b[1;31m";
const COLOR_RESET: &str = "\x1b[0m";

fn main() -> Result<(), Box<dyn Error>> {
    // let app_name = "My Crafter App";
    // let package_name = "com.crafter.mycrafterapp";
    let argv: Vec<String> = args().collect();
    let argc = argv.len();
    let program_name = Path::new(&argv[0])
        .file_stem().unwrap().to_str().unwrap().to_string();
    if argc > 1 {
        match argv[1].as_str() {
            "-h" | "--help" => print_usage(&program_name),
            "-V" | "--version" => println!("{program_name} {VERSION}"),
            "create" => {
                if argc < 4 {
                    eprintln!("{COLOR_RED_BOLD}insufficient arguments for subcommand \"create\"{COLOR_RESET}\n");
                    print_usage(&program_name);
                } else {
                    create_project(&argv[2], &argv[3])?;
                }
            } 
            x => {
                eprintln!("{COLOR_RED_BOLD}Unknown subcommand or option \"{x}\"{COLOR_RESET}\n");
                print_usage(&program_name);
            }
        }
    } else {
        print_usage(&program_name);
    }
    Ok(())
}

fn print_usage(program_name: &String) {
    let usage = format!(r#"Description:
  CLI tool to manage android projects

Usage: 
  {program_name} <SUBCOMMAND>
  {program_name} [OPTIONS]

Subcommands:
  create <APP_NAME> <PACKAGE_NAME>  Create new android project

Options:
  -h, --help           Print help
  -V, --version        Print version"#);
    println!("{}", usage);
}

fn create_project(app_name: &str, package_name: &str) -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let root_path = Path::new(app_name);
    if root_path.exists() {
        eprintln!("{COLOR_RED_BOLD}A folder with the app name \"{app_name}\" already exists.");
        eprintln!("Choose a different name or rename the existing folder.{COLOR_RESET}");
        return Ok(());
    }
    println!("Creating new android project...\nAPP_NAME={app_name}\nPACKAGE_NAME={package_name}");
    let gradle_path = root_path.join("gradle");
    let app_path = root_path.join("app");
    let src_main_path = app_path.join("src").join("main");
    let src_java_path = src_main_path.join("java");
    let src_values_path = src_main_path.join("res").join("values");
    let mut src_innermost_path = src_java_path;
    for part in package_name.split('.') {
        src_innermost_path = src_innermost_path.join(part);
    }
    let src_theme_path = src_innermost_path.join("ui").join("theme");

    // create all the directories and subdirectories
    println!("Creating all the directories and subdirectories...");
    create_dir(&root_path)?;
    create_dir(&gradle_path)?;
    create_dir(&app_path)?;
    create_dir_all(&src_values_path)?;
    create_dir_all(&src_innermost_path)?;
    create_dir_all(&src_theme_path)?;
    
    // generate the sources
    println!("Generating the sources...");
    let sources = Sources::generate(app_name, package_name);
    write(root_path.join("gradle.properties"), sources.gradle_properties)?;
    write(root_path.join(".gitignore"), sources.root_gitignore)?;
    write(app_path.join(".gitignore"), sources.app_gitignore)?;
    write(gradle_path.join("libs.versions.toml"), sources.libs_versions_toml)?;

    // write xml files
    write(src_main_path.join("AndroidManifest.xml"), sources.android_manifest_xml)?;
    write(src_values_path.join("resources.xml"), sources.resources_xml)?;

    // write kotlin files
    write(src_innermost_path.join("MainActivity.kt"), sources.main_activity_kt)?;
    write(src_theme_path.join("Color.kt"), sources.color_kt)?;
    write(src_theme_path.join("Theme.kt"), sources.theme_kt)?;
    write(src_theme_path.join("Type.kt"), sources.type_kt)?;

    // write gradle files
    write(root_path.join("settings.gradle.kts"), sources.settings_gradle_kts)?;
    write(root_path.join("build.gradle.kts"), sources.root_build_gradle_kts)?;
    write(app_path.join("build.gradle.kts"), sources.app_build_gradle_kts)?;
    
    println!("DONE in {}ms.", now.elapsed().as_millis());

    Ok(())
}


