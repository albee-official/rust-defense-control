pub trait BitInspect<T> {
    fn is_bit_set(&self, bit: usize) -> bool;
    fn is_bit_cleared(&self, bit: usize) -> bool;
    fn set_bit(self, bit: usize) -> Self;
    fn clear_bit(self, bit: usize) -> Self;
}

impl BitInspect<u8> for u8 {
    fn is_bit_set(&self, bit: usize) -> bool {
        self & (1u8 << bit) != 0
    }

    fn is_bit_cleared(&self, bit: usize) -> bool {
        self & (1u8 << bit) == 0
    }

    fn set_bit(mut self, bit: usize) -> Self {
        self |= 1u8 << bit;
        self
    }

    fn clear_bit(mut self, bit: usize) -> Self {
        self |= 1u8 << bit;
        self
    }
}
