pub type Trit = i8;

pub const HASH_LENGTH: usize = 243;
pub const STATE_LENGTH: usize = HASH_LENGTH * 3;
pub const TRUTH_TABLE: [Trit; 11] = [1, 0, -1, 2, 1, -1, 0, 2, -1, 1, 0];

/// Ptrit.0 Ptrit.1 Trit
/// 0       0       NA
/// 0       1       1
/// 1       0       -1
/// 1       1       0
#[derive(Clone, Copy, Debug)]
pub struct Ptrit(pub u8, pub u8);

impl Default for Ptrit {
    fn default() -> Self {
        Ptrit(255, 255)
    }
}