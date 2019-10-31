use crate::constants::*;
use crate::Sponge;
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

impl Sponge for Curl
where
    Self: Send + 'static,
{
    type Item = Trit;

    fn absorb(&mut self, trits: &[Self::Item]) {
        for c in trits.chunks(HASH_LENGTH) {
            self.state[0..c.len()].copy_from_slice(c);
            self.transform();
        }
    }

    fn squeeze(&mut self, out: &mut [Self::Item]) {
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

    fn reset(&mut self) {
        self.state = [0; STATE_LENGTH];
    }
}

impl Curl {
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

        for round in 0..self.rounds {
            let (state_out, state) = if round % 2 == 0 {
                (&mut local_state, &self.state)
            } else {
                (&mut self.state, &local_state)
            };

            for state_index in 0..STATE_LENGTH {
                let idx: usize = (state[TRANSFORM_INDICES[state_index]] as usize)
                    .wrapping_add((state[TRANSFORM_INDICES[state_index + 1]] as usize) << 2)
                    .wrapping_add(5);

                state_out[state_index] = TRUTH_TABLE[idx];
            }
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