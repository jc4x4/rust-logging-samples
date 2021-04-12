use log::debug;
use log::error;
use log::info;
use log::warn;

#[allow(dead_code)]
fn main() {
    env_logger::init();
    debug!("Mary has a little lamb");
    error!("{}", "Its fleece was white as snow");
    info!("{:?}", "And every where that Mary went");
    warn!("{:#?}", "The lamb was sure to go");
}
