use crate::constants::*;
use core::fmt;

#[derive(Clone, Copy)]
pub struct Curl {
    rounds: usize,
    state: [Trit; STATE_LENGTH],
}

impl Default for Curl {
    fn default() -> Curl {
        Curl {
            rounds: 81,
            state: [0; STATE_LENGTH],
        }
    }
}

impl fmt::Debug for Curl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Curl: [rounds: [{}], state: {:?}",
            self.rounds,
            &self.state[..],
        )
    }
}

impl Curl {
    /// Absorb trits into the sponge
    pub fn absorb(&mut self, trits: &[Trit]) {
        for c in trits.chunks(HASH_LENGTH) {
            self.state[0..c.len()].copy_from_slice(c);
            unsafe {
                self.unsafe_transform();
            }
        }
    }

    /// Squeeze trits out of the sponge and copy them into `out`
    pub fn squeeze(&mut self, out: &mut [Trit]) {
        let trit_count = out.len();
        let hash_count = trit_count / HASH_LENGTH;

        for c in out.chunks_mut(HASH_LENGTH) {
            c.copy_from_slice(&self.state[0..HASH_LENGTH]);
            unsafe {
                self.unsafe_transform();
            }
        }

        let last = trit_count - hash_count * HASH_LENGTH;
        out[trit_count - last..].copy_from_slice(&self.state[0..last]);
        if trit_count % HASH_LENGTH != 0 {
            unsafe {
                self.unsafe_transform();
            }
        }
    }

    /// Reset the sponge to initial state
    pub fn reset(&mut self) {
        self.state = [0; STATE_LENGTH];
    }

    /// Digest inputs and then compute the hash with length of provided output slice
    pub fn digest(&mut self, input: &[Trit], output: &mut [Trit]) {
        self.absorb(input);
        self.squeeze(output);
    }

    // TODO: return with Result, define proper error type
    pub fn new(rounds: usize) -> Curl {
        let mut curl = Curl::default();
        curl.rounds = rounds;
        curl
    }

    pub fn state(&self) -> &[Trit] {
        &self.state
    }

    #[allow(dead_code)]
    fn transform(&mut self) {
        let mut scratchpad_index = 0;
        let mut local_state: [Trit; STATE_LENGTH] = [0; STATE_LENGTH];

        for _ in 0..self.rounds {
            local_state.copy_from_slice(&self.state);

            for state_index in 0..STATE_LENGTH {
                let prev_scratchpad_index = scratchpad_index;
                if prev_scratchpad_index < 365 {
                    scratchpad_index += 364;
                } else {
                    scratchpad_index -= 365;
                }

                let idx: usize = (local_state[prev_scratchpad_index]
                    + (local_state[scratchpad_index] << 2)
                    + 5) as usize;

                self.state[state_index] = TRUTH_TABLE[idx];
            }
        }
    }
    
    unsafe fn unsafe_transform(&mut self) {
        let mut local_state: [Trit; STATE_LENGTH] = [0; STATE_LENGTH];
        local_state.copy_from_slice(&self.state);

        let mut t: *mut i8;
        let mut s1 = self.state.as_mut_ptr();
        let mut s2 = local_state.as_mut_ptr();

        for _ in 0..self.rounds {
            *s1 = TRUTH_TABLE[(*s2 + (*s2.offset(364) << 2) + 5) as usize];

            for i in 0..364 {
                *s1.offset(2 * i + 1) = TRUTH_TABLE
                    [(*s2.offset(364 - i) + (*s2.offset(729 - (i + 1)) << 2) + 5) as usize];
                *s1.offset(2 * i + 2) = TRUTH_TABLE
                    [(*s2.offset(729 - (i + 1)) + (*s2.offset(364 - (i + 1)) << 2) + 5) as usize];
            }

            t = s1;
            s1 = s2;
            s2 = t;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curl_works() {
        let transaction = [0i8; 8019];
        let mut tx_hash = [0i8; 243];
        let mut curl = Curl::default();
        curl.digest(&transaction, &mut tx_hash);
    }
}
