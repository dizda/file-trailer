use clap::{AppSettings, Clap};
use std::path::PathBuf;

/// # File-trailer
///
/// File trailer will listen a log file then:
///
///  - publish any new message to AMQP
///  - rotate logs automatically
///
#[derive(Debug, clap::Clap, Clone)]
#[clap(name = "file-trailer")]
pub struct Opt {
    /// Override the config file
    #[clap(parse(from_os_str), short, long, env)]
    pub file: PathBuf,

    /// If the filesize go beyond that value, the file will get rotated
    /// value is in bytes
    #[clap(short, long, default_value = "20000000", env)]
    pub max_filesize: u64,

    /// Check if the file needs to be rotated
    /// value in seconds
    #[clap(short, long, default_value = "5", env)]
    pub rotate_file_interval: u64,

    /// Rotated files will have a date on their filenames,
    /// can change the current structure `%Y-%m-%d-%H-%M-%S`
    #[clap(short, long)]
    pub date_format: Option<String>,

    /// This is the capacity of the publish queue
    /// If it's set to 1, it will wait for amqp to finish publish the only message in the buffer
    /// before accepting new one.
    ///
    /// Which is conservative but not concurrent.
    #[clap(long, default_value = "1", env)]
    pub buffer_publish: usize,

    /// Uri of the AMQP server to publish to
    #[clap(long, default_value = "amqp://guest:guest@127.0.0.1:5672/%2f", env)]
    pub amqp_uri: String,

    #[clap(long, env)]
    pub amqp_exchange: String,

    #[clap(long, env)]
    pub amqp_routing_key: String,
}

pub fn parse() -> Opt {
    Opt::parse()
}
