use structopt;
use structopt::StructOpt;
use std::path::PathBuf;
use mbox_reader::MboxFile;

#[derive(Debug, StructOpt)]
struct Options {
    /// The mbox file to import from
    #[structopt(name = "MBOX")]
    pub mbox_path: PathBuf,
    /// Take only the first n (all by default)
    #[structopt(long = "take")]
    pub take: Option<usize>,
    #[structopt(long = "skip", default_value = "0")]
    pub skip: usize,
}

fn main() {
    let options = Options::from_args();

    let mbox_file = MboxFile::from_file(options.mbox_path.as_path())
        .expect(&format!("Failed to read mbox {:?}", options.mbox_path));
    let take_amount = options.take.unwrap_or(std::usize::MAX);
    let skip_amount = options.skip;

    for email in mbox_file.iter().skip(skip_amount).take(take_amount) {
        println!("Start: {}", email.start().as_str());

        let raw_message= email.message().unwrap_or("<No Message>".as_bytes());
        let message= String::from_utf8(Vec::from(raw_message))
            .unwrap_or(String::from("<binary>"));
        println!("Message: {}", message);
    }
}
