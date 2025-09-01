use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::args::Args;

const PROJECT_DIR: &str = "mt-project";
const CONFIG_DIR: &str = "config";
const FILE_1_NAME: &str = "file-1.txt";
const FILE_2_NAME: &str = "file-2.txt";

/// this struct represents the data that will be used to perform the operations
#[allow(dead_code)]
pub struct Config {
    /// this field represents the path to the first of two possible files from which
    /// to read contents
    file_1: PathBuf,
    /// this field represents the path to the second of two possibles files from which
    /// to read contents
    file_2: PathBuf,
}

impl Config {
    /// returns an instance of `Config`. When first instanced, the field `data` is defined
    /// as an empty `String`.
    pub fn new() -> Self {
        let (file_1, file_2) = Self::set_file_paths();

        Self { file_1, file_2 }
    }

    /// returns a tuple that represents the paths to both of the files from
    /// which the program can read contents.
    /// The program assumes that both the config dir and the files exist and
    /// does not perform any checks nor does it attempt to create the files
    /// in case they do not exist. This is a conscious decision and is done
    /// so out of convenience in order to be able to focus on the task at hand.
    /// The program also assumes that it will only be run using `cargo run` or,
    /// in other words, from the default dir.
    fn set_file_paths() -> (PathBuf, PathBuf) {
        let mut cwd = env::current_exe().expect("Could not get cwd");

        let config = loop {
            cwd = cwd.parent().expect("Cold not get parent dir").to_path_buf();

            if cwd.ends_with(PROJECT_DIR) {
                break cwd.join(CONFIG_DIR);
            }
        };

        let file_1 = PathBuf::from(&config.join(FILE_1_NAME));
        let file_2 = PathBuf::from(&config.join(FILE_2_NAME));

        (file_1, file_2)
    }
}

impl Config {
    /// gets the path to the first file from which the program can read data
    fn get_file_1_path(&self) -> &Path {
        &self.file_1.as_path()
    }

    /// gets the path to the second file from which the program can read data
    fn get_file_2_path(&self) -> &Path {
        &self.file_2.as_path()
    }

    /// loads the contents of a file pointed to by 'filepath' and returns its contents
    /// as a `String`
    fn load_data(&self, filepath: &Path) -> String {
        fs::read_to_string(filepath).expect("Could not read file contents")
    }

    /// fetches the contents of the file that corresponds to the arg that the program is being
    /// run with. If the file retrieved is empty, which should not be the case,
    /// the function will return `None`, but otherwise will return `Some(String)`.
    pub fn run(&self, arg: Args) -> Option<String> {
        let data = match arg {
            Args::P => self.load_data(self.get_file_1_path()),
            Args::S => self.load_data(self.get_file_2_path()),
        };

        if data.is_empty() { None } else { Some(data) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_test_outputs_expected_files() {
        let (file_1, file_2) = Config::set_file_paths();

        assert!(file_1.ends_with(FILE_1_NAME));
        assert!(file_2.ends_with(FILE_2_NAME));
    }

    #[test]
    fn positive_test_filepath_fields_are_correctly_initialised() {
        let config = Config::new();

        assert!(config.file_1.ends_with(FILE_1_NAME));
        assert!(config.file_2.ends_with(FILE_2_NAME));
    }

    #[test]
    fn positive_test_retrieves_the_expected_fields() {
        let config = Config::new();

        assert!(config.get_file_1_path().ends_with(FILE_1_NAME));
        assert!(config.get_file_2_path().ends_with(FILE_2_NAME));
    }

    #[test]
    fn positive_test_file_is_not_empty() {
        let config = Config::new();

        assert_ne!(config.run(Args::S), None);
        assert_ne!(config.run(Args::P), None);
    }
}
