use crate::constants::*;
use core::fmt;

#[derive(Clone, Copy)]
pub struct Curl {
    num_rounds: usize,
    state: [i8; STATE_LENGTH],
}

impl Default for Curl {
    fn default() -> Curl {
        Curl {
            num_rounds: 81,
            state: [0; STATE_LENGTH],
        }
    }
}

impl fmt::Debug for Curl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Curl: [rounds: [{}], state: {:?}",
            self.num_rounds,
            &self.state[..],
        )
    }
}