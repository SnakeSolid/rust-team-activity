mod activity;

use self::activity::ActivityHandler;

use iron::Iron;
use mount::Mount;
use staticfile::Static;

use Config;
use Database;

pub fn start(config: &Config, database: Database) -> () {
    let mut mount = Mount::new();
    mount.mount("/api/v1/activity", ActivityHandler::new(config, database));
    mount.mount("/static", Static::new("public/static"));
    mount.mount("/", Static::new("public"));

    let server = config.server();
    let address = server.address();
    let port = server.port();

    println!("Listening on {}:{}...", address, port);

    match Iron::new(mount).http((address, port)) {
        Ok(_) => {}
        Err(err) => error!("Failed to start HTTP server: {}", err),
    }
}
