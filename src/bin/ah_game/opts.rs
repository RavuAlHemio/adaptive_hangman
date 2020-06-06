use clap::Clap;

#[derive(Clap)]
#[clap()]
pub(crate) struct Opts {
    #[clap(short = "D", long = "debug", about = "Activates debug output.")]
    pub debug: bool,

    #[clap(short = "d", long = "dict", default_value = "dict.txt", value_names = &["DICTFILE"], about = "Specifies the dictionary file to read.")]
    pub dict_path: String,

    #[clap(short = "p", long = "pattern", value_names = &["PATTERN"], about = "Specifies a specific pattern of words to choose.")]
    pub pattern: Option<String>,

    #[clap(short = "r", long = "randomly-remove", default_value = "0", value_names = &["PERCENT"], about = "Percentage of words to randomly remove from dictionary.")]
    pub remove_percentage: f64,

    #[clap(short = "m", long = "min-words", value_names = &["NUMBER"], about = "Removes patterns matched by fewer than this number of words.")]
    pub min_words: Option<usize>,

    #[clap(short = "l", long = "lives", default_value = "8", value_names = &["NUMBER"], about = "Number of lives.")]
    pub lives: u64,
}
