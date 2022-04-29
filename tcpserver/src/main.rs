mod router;
mod server;

use server::Server;

fn main() {
    Server.run();
}
