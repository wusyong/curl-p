use crate::constants::*;
use core::fmt;

#[derive(Clone, Copy)]
pub struct Curl {
    rounds: usize,
    state: [Ptrit; STATE_LENGTH],
}

impl Default for Curl {
    fn default() -> Curl {
        Curl {
            rounds: 81,
            state: [Ptrit::default(); STATE_LENGTH],
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
    pub fn absorb(&mut self, trits: &[Ptrit]) {
        for c in trits.chunks(HASH_LENGTH) {
            self.state[0..c.len()].copy_from_slice(c);
            self.transform();
        }
    }

    pub fn squeeze(&mut self) ->  [Ptrit; HASH_LENGTH] {
        let mut output: [Ptrit; HASH_LENGTH] = [Ptrit::default(); HASH_LENGTH];
        self.squeeze_into(&mut output);
        output
    }

    /// Squeeze trits out of the sponge and copy them into `out`
    pub fn squeeze_into(&mut self, out: &mut [Ptrit]) {
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
        self.state = [Ptrit::default(); STATE_LENGTH];
    }

    pub fn digest(&mut self, input: &[Ptrit]) ->  [Ptrit; HASH_LENGTH] {
        self.absorb(input);
        self.squeeze()
    }

    /// Digest inputs and then compute the hash with length of provided output slice
    pub fn digest_into(&mut self, input: &[Ptrit], output: &mut [Ptrit]) {
        self.absorb(input);
        self.squeeze_into(output);
    }

    // TODO: return with Result, define proper error type
    pub fn new(rounds: usize) -> Curl {
        let mut curl = Curl::default();
        curl.rounds = rounds;
        curl
    }

    pub fn state(&self) -> &[Ptrit] {
        &self.state
    }
    
    fn transform(&mut self) {
        let mut local_state: [Ptrit; STATE_LENGTH] = [Ptrit::default(); STATE_LENGTH];
        let mut local_index = 0;

        for _ in 0..self.rounds {
            local_state.copy_from_slice(&self.state);

            for state_index in 0..STATE_LENGTH {
                let a = self.state[local_index].0;
                let b = self.state[local_index].1;
    
                if local_index < 365 {
                    local_index += 364;
                } else {
                    local_index -= 365;
                }
    
                let d = b ^ local_state[local_index].0;
                self.state[state_index].0 = !(d & a);
                self.state[state_index].1 = (a ^ local_state[local_index].1) | self.state[state_index].0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pcurl_digest() {
        let transaction = [Ptrit::default(); 8019];
        let mut tx_hash1 = [Ptrit::default(); 243];
        let mut curl = Curl::default();
        curl.digest_into(&transaction, &mut tx_hash1);
        curl.reset();
        let tx_hash2 = curl.digest(&transaction);

        for i in 0..243 {
            assert_eq!(tx_hash1[i].0, tx_hash2[i].0);
            assert_eq!(tx_hash1[i].1, tx_hash2[i].1);
        }
    }
}
