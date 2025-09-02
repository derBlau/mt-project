/// This struct represents the data extracted from the file in the
/// current state of the program.
pub struct Data {
    /// this field represents the contents extracted from the file
    data: String,
    /// this field represents the amount of words that were extracted
    /// from the file
    word_count: usize,
}

impl Data {
    pub fn new(data: String) -> Self {
        let word_count = {
            let words: Vec<&str> = data.split_whitespace().collect();

            words.len()
        };

        Self { data, word_count }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_test_corectly_counts_the_number_of_words_in_string() {
        let data = Data::new(String::from("I am a test string"));

        assert_eq!(data.word_count, 5);
    }
}
