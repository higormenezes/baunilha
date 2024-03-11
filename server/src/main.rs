use server::WebServer;

fn main() {
    WebServer::start().expect("Web server fail to start");
}
