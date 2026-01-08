use ctor::ctor;
mod protocol;
mod opts;

// CLEAN CODE WOW
#[ctor]
fn main() {
    unsafe {
        protocol::init_moonwave_url_protocol();
    }
}
