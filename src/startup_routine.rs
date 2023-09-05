use std::{
    env::{self, VarError},
    fs,
    path::Path,
};
pub fn get_home_path() -> Result<String, VarError> {
    env::var("HOME")
}
pub fn run() -> anyhow::Result<()> {
    create_storage_directory()?;
    create_storage_file()?;
    Ok(())
}
fn check_if_storage_directory_exists(home_path: &str) -> bool {
    Path::new(&format!("{}/.leitner_box_rs", home_path)).is_dir()
}
fn create_storage_directory() -> anyhow::Result<()> {
    let home_path = get_home_path()?;
    if !check_if_storage_directory_exists(&home_path) {
        fs::create_dir(&format!("{}/.leitner_box_rs", home_path))?;
    }
    Ok(())
}
fn check_if_storage_file_exists(home_path: &str) -> bool {
    Path::new(&format!("{}/.leitner_box_rs/data.toml", home_path)).is_file()
}
fn create_storage_file() -> anyhow::Result<()> {
    let home_path = get_home_path()?;
    if !check_if_storage_file_exists(&home_path) {
        fs::write(&format!("{}/.leitner_box_rs/data.toml", home_path), "")?;
    }
    Ok(())
}
