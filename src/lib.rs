use crate::handshake::AppInfo;

mod server;
mod handshake;
mod osc;

pub fn start_server() -> bool {
    // New server
    let server = server::Server::new(
        &AppInfo {
            id: "test",
            friendly_name: "Test Server",
            version: "0.0.1",
        },
        "127.0.0.1",
        25565,
        9000
    ).unwrap();
    server.commander_thread.join().unwrap();
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_server();
    }
}
