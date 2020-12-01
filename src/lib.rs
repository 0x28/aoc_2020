use std::path::PathBuf;

pub fn input_file(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR").to_owned() + "/input/" + filename)
}
