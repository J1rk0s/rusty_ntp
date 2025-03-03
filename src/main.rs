use rusty_ntp::NtpServer;

use clap::{arg, command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    ip: String,

    #[arg(short, long, default_value_t = 123)]
    port: u16,
}

fn main() {
    let args = Args::parse();

    NtpServer::init(args.ip, args.port).start();
}