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
            self.transform();
        }
    }

    pub fn squeeze(&mut self) ->  [Trit; HASH_LENGTH] {
        let mut output: [Trit; HASH_LENGTH] = [0; HASH_LENGTH];
        self.squeeze_into(&mut output);
        output
    }

    /// Squeeze trits out of the sponge and copy them into `out`
    pub fn squeeze_into(&mut self, out: &mut [Trit]) {
        let trit_count = out.len();
        let hash_count = trit_count / HASH_LENGTH;

        for c in out.chunks_mut(HASH_LENGTH) {
            c.copy_from_slice(&self.state[0..HASH_LENGTH]);
            self.transform();
        }

        let last = trit_count - hash_count * HASH_LENGTH;
        out[trit_count - last..].copy_from_slice(&self.state[0..last]);
        if trit_count % HASH_LENGTH != 0 {
            self.transform();
        }
    } 

    /// Reset the sponge to initial state
    pub fn reset(&mut self) {
        self.state = [0; STATE_LENGTH];
    }

    pub fn digest(&mut self, input: &[Trit]) ->  [Trit; HASH_LENGTH] {
        self.absorb(input);
        self.squeeze()
    }

    /// Digest inputs and then compute the hash with length of provided output slice
    pub fn digest_into(&mut self, input: &[Trit], output: &mut [Trit]) {
        self.absorb(input);
        self.squeeze_into(output);
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
    
    fn transform(&mut self) {
        let mut local_state: [Trit; STATE_LENGTH] = [0; STATE_LENGTH];
        local_state.copy_from_slice(&self.state);

        let mut s1 = self.state.as_mut_ptr();
        let mut s2 = local_state.as_mut_ptr();

        unsafe {
            for _ in 0..self.rounds {
                *s1 = TRUTH_TABLE[(*s2 + (*s2.offset(364) << 2) + 5) as usize];

                for i in 0..364 {
                    *s1.offset(2 * i + 1) = TRUTH_TABLE
                        [(*s2.offset(364 - i) + (*s2.offset(729 - (i + 1)) << 2) + 5) as usize];
                    *s1.offset(2 * i + 2) = TRUTH_TABLE
                        [(*s2.offset(729 - (i + 1)) + (*s2.offset(364 - (i + 1)) << 2) + 5) as usize];
                }

                core::mem::swap(&mut s1, &mut s2);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curl_digest() {
        let transaction = [0i8; 8019];
        let mut tx_hash1 = [0i8; 243];
        let mut curl = Curl::default();
        curl.digest_into(&transaction, &mut tx_hash1);
        curl.reset();
        let tx_hash2 = curl.digest(&transaction);

        for i in 0..243 {
            assert_eq!(tx_hash1[i], tx_hash2[i]);
        }
    }
}
