mod sources;

use std::error::Error;
use std::fs::{create_dir, create_dir_all, write};
use std::path::Path;

use crate::sources::Sources;

fn main() -> Result<(), Box<dyn Error>> {
    let app_name = "My Crafter App";
    let package_name = "com.crafter.mycrafterapp";
    let root_path = Path::new(app_name);
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
    create_dir(&root_path)?;
    create_dir(&gradle_path)?;
    create_dir(&app_path)?;
    create_dir_all(&src_values_path)?;
    create_dir_all(&src_innermost_path)?;
    create_dir_all(&src_theme_path)?;
    
    // generate the sources
    let sources = Sources::generate(app_name, package_name);
    write(
        root_path.join("gradle.properties"),
        sources.gradle_properties
    )?;
    write(
        root_path.join(".gitignore"),
        sources.root_gitignore
    )?;
    write(
        app_path.join(".gitignore"),
        sources.app_gitignore
    )?;
    write(
        gradle_path.join("libs.versions.toml"),
        sources.libs_versions_toml
    )?;

    // write xml files
    write(
        src_main_path.join("AndroidManifest.xml"),
        sources.android_manifest_xml
    )?;
    write(
        src_values_path.join("resources.xml"),
        sources.resources_xml
    )?;

    // write kotlin files
    write(
        src_innermost_path.join("MainActivity.kt"),
        sources.main_activity_kt
    )?;
    write(
        src_theme_path.join("Color.kt"),
        sources.color_kt
    )?;
    write(
        src_theme_path.join("Theme.kt"),
        sources.theme_kt
    )?;
    write(
        src_theme_path.join("Type.kt"),
        sources.type_kt
    )?;

    // write gradle files
    write(
        root_path.join("settings.gradle.kts"),
        sources.settings_gradle_kts
    )?;
    write(
        root_path.join("build.gradle.kts"),
        sources.root_build_gradle_kts
    )?;
    write(
        app_path.join("build.gradle.kts"),
        sources.app_build_gradle_kts
    )?;
    Ok(())
}
