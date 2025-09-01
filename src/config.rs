use std::env;
use std::path::PathBuf;

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
    fn set_file_paths() -> (PathBuf, PathBuf) {
        let mut cwd = env::current_exe().expect("Could not get cwd");

        let config = loop {
            cwd = cwd.parent().expect("Cold not get parent dir").to_path_buf();

            if cwd.ends_with("mt-project/config") {
                break cwd;
            }
        };

        let file_1 = PathBuf::from(&config.join("file-1.txt"));
        let file_2 = PathBuf::from(&config.join("file-2.txt"));

        (file_1, file_2)
    }
}
