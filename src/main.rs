use structopt;
use structopt::StructOpt;
use std::path::PathBuf;
use mbox_reader::MboxFile;
use native_tls::TlsConnector;

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
    #[structopt(long = "username", default_value = "")]
    pub username: String,
    #[structopt(long = "password", default_value = "")]
    pub password: String,
    #[structopt(long = "domain", default_value = "imap.example.com")]
    pub domain: String,
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

    let username = options.username;
    let password = options.password;
    let domain = options.domain.as_str();

    let tls = TlsConnector::builder().build().expect("Failed to build TlsConnector");
    let client = imap::connect((domain, 993), domain, &tls)
        .map_err(|error| eprintln!("Failed to connect with error: {}", error))
        .unwrap();

    let mut session = client.login(username, password)
        .map_err(|(error, _)| eprintln!("Failed to login with error: {}", error))
        .unwrap();

    let mailbox = "INBOX";
    session.select(mailbox)
        .map_err(|error| eprintln!("Failed selecting mailbox {} with error: {}", mailbox, error))
        .unwrap();

    let messages = session.fetch("1", "ALL")
        .map_err(|error| eprintln!("Failed to fetch emails with error: {}", error))
        .unwrap();

    for message in messages.iter() {
        println!("{:?}", message);
    }

    session.logout()
        .map_err(|error| eprintln!("Failed logging out with error: {}", error))
        .unwrap();
}
