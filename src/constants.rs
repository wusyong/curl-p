pub type Trit = i8;

pub const HASH_LENGTH: usize = 243;
pub const STATE_LENGTH: usize = HASH_LENGTH * 3;
pub const TRUTH_TABLE: [Trit; 11] = [1, 0, -1, 2, 1, -1, 0, 2, -1, 1, 0];
