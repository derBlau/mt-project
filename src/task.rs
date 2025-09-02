use crate::args::Args;

const P_WORD: &str = "replaced";
const S_CHAR: &str = "th";

/// This struct represents the operation that will be perform
/// on the data that was extracted from the file. It holds a
/// field with a function pointer.
/// The task should be fed to a task queue that the worker threads
/// can access.
pub struct Task {
    /// this field represents the operation that will be perform
    /// on the data that was extracted from the file
    operation: fn(String) -> String,
}

impl Task {
    /// finds each word that begins with the char `p` and replaces
    /// it with another word
    pub fn target_p(data: String) -> String {
        let change_word = |word: &str| {
            if word.starts_with('p') {
                P_WORD.to_string()
            } else {
                word.to_string()
            }
        };

        let words: Vec<String> = data.split_whitespace().map(change_word).collect();

        words.join(" ")
    }

    /// finds each occurrence of the char `s` and replaces it with
    /// a given String
    pub fn target_s(data: String) -> String {
        let parse_word = |word: &str| {
            let word_chars: Vec<String> = word
                .chars()
                .map(|c| {
                    if c == 's' {
                        S_CHAR.to_string()
                    } else {
                        c.to_string()
                    }
                })
                .collect();

            word_chars.join("")
        };

        let words: Vec<String> = data.split_whitespace().map(parse_word).collect();

        words.join(" ")
    }

    /// returns an instance of `Task`
    pub fn new(arg: Args) -> Self {
        let operation = match arg {
            Args::P => Self::target_p,
            Args::S => Self::target_s,
        };

        Self { operation }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn positive_test_can_replace_words_that_begin_with_p() {
        let test_string_1 = String::from("perro");
        let test_string_2 = String::from("a perro");
        let test_string_3 = String::from("a small apricot");

        assert_eq!(Task::target_p(test_string_1), String::from("replaced"));
        assert_eq!(Task::target_p(test_string_2), String::from("a replaced"));
        assert_eq!(
            Task::target_p(test_string_3),
            String::from("a small apricot")
        );
    }

    #[test]
    fn positive_test_can_replace_s_char_in_word() {
        let test_string_1 = String::from("si");
        let test_string_2 = String::from("I see that");
        let test_string_3 = String::from("I know");

        assert_eq!(Task::target_s(test_string_1), String::from("thi"));
        assert_eq!(Task::target_s(test_string_2), String::from("I thee that"));
        assert_eq!(Task::target_s(test_string_3), String::from("I know"));
    }
}
