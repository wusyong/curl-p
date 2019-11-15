use crate::constants::*;

pub fn digest(trits: &[i8], num_rounds: usize) -> [i8; HASH_LENGTH] {
    let mut state = [0i8; STATE_LENGTH];

    for i in (0..trits.len()).step_by(HASH_LENGTH) {
        state[0..HASH_LENGTH].copy_from_slice(&trits[i..i + HASH_LENGTH]);
        unsafe {
            transform(&mut state, num_rounds);
        }
    }

    let mut hash = [0i8; HASH_LENGTH];
    hash.copy_from_slice(&state[..HASH_LENGTH]);
    hash
}

unsafe fn transform(state: &mut [i8; STATE_LENGTH], num_rounds: usize) {
    let mut state2 = [0i8; STATE_LENGTH];
    state2.copy_from_slice(state);

    let mut t: *mut i8;
    let mut s1 = state.as_mut_ptr();
    let mut s2 = state2.as_mut_ptr();

    for _ in 0..num_rounds {
        *s1 = TRUTH_TABLE[(*s2 + (*s2.offset(364) << 2) + 5) as usize];

        for i in 0..364 {
            *s1.offset(2 * i + 1) =
                TRUTH_TABLE[(*s2.offset(364 - i) + (*s2.offset(729 - (i + 1)) << 2) + 5) as usize];
            *s1.offset(2 * i + 2) = TRUTH_TABLE
                [(*s2.offset(729 - (i + 1)) + (*s2.offset(364 - (i + 1)) << 2) + 5) as usize];
        }

        t = s1;
        s1 = s2;
        s2 = t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curl_works() {
        let transaction = [0i8; 8019];
        let _output = digest(&transaction, 81);
    }
}
