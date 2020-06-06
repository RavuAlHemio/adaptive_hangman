use clap::Clap;

#[derive(Clap)]
#[clap()]
pub(crate) struct Opts {
    #[clap(short = "d", long = "dict", default_value = "dict.txt", value_names = &["DICTFILE"], about = "Specifies the dictionary file to read.")]
    pub dict_path: String,

    #[clap(short = "l", long = "lives", default_value = "8", value_names = &["NUMBER"], about = "Number of lives.")]
    pub lives: u64,

    #[clap(required = true, value_names = &["PATTERN"], about = "The pattern whose word to guess.")]
    pub pattern: String,
}
