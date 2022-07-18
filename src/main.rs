//! Demo risingcloud webtask
mod req;
mod scary_logic;

/// Start running the webtask!
fn main() {
    let caller = req::IncomingRequest::load().unwrap();
    let response = scary_logic::Response::from(caller);
    response.save().unwrap();
}
