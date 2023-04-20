mod errors;

pub use errors::{Error, Result};

mod normal;
mod random;

pub use random::RandomKill;