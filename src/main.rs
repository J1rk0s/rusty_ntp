use rusty_ntp::NtpServer;

fn main() {
    NtpServer::init("127.0.0.1".to_owned(), 123).start();
}