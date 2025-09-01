use std::env;
use std::path::PathBuf;

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
    /// this field represents the buffer for the contents of the target file.
    data: String,
}

impl Config {
    /// returns an instance of `Config`. When first instanced, the field `data` is defined
    /// as an empty `String`.
    pub fn new() -> Self {
        let (file_1, file_2) = Self::set_file_paths();
        let data = String::new();

        Self {
            file_1,
            file_2,
            data,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_case_test_outputs_expected_files() {
        let (file_1, file_2) = Config::set_file_paths();

        assert!(file_1.ends_with(FILE_1_NAME));
        assert!(file_2.ends_with(FILE_2_NAME));
    }

    #[test]
    fn positive_case_data_field_is_empty_upon_initialisation() {
        let config = Config::new();

        assert!(config.data.is_empty());
    }

    #[test]
    fn positive_case_filepath_fields_are_correctly_initialised() {
        let config = Config::new();

        assert!(config.file_1.ends_with(FILE_1_NAME));
        assert!(config.file_2.ends_with(FILE_2_NAME));
    }
}
