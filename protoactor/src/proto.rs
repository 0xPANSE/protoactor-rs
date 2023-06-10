pub use actor::*;

mod actor {
    use std::hash::Hash;
    include!(concat!(env!("OUT_DIR"), "/actor.rs"));

    impl Pid {
        pub fn new(id: String, address: String) -> Self {
            Self {
                id,
                address,
                request_id: 0,
            }
        }

        pub fn with_request_id(mut self, request_id: u32) -> Self {
            self.request_id = request_id;
            self
        }
    }

    impl Hash for Pid {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            if !self.id.is_empty() {
                self.id.hash(state);
            }
            if !self.address.is_empty() {
                self.address.hash(state);
            }
            if self.request_id != 0 {
                self.request_id.hash(state);
            }
        }
    }
}
