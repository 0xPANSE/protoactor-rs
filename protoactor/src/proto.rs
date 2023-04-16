mod actor {
    include!(concat!(env!("OUT_DIR"), "/actor.rs"));
}

pub use actor::*;
