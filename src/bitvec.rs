pub struct BitVec {
    data: Vec<u32>,
    nbits: usize
}

impl BitVec {
    pub fn new(bits: usize) -> BitVec {
        let words = u32s(bits);
        BitVec { data: vec![0; words], nbits: words * 32 }
    }

    pub fn ones(bits: usize) -> BitVec {
        let words = u32s(bits);
        BitVec { data: vec![1; words], nbits: words * 32 }
    }

    #[inline]
    pub fn set(&mut self, bit: usize) {
        assert!(bit < self.nbits);

        let word = bit / 32;
        let b = bit % 32;
        let flag = 1 << b;

        self.data[word] = self.data[word] | flag;
    }

    #[inline]
    pub fn unset(&mut self, bit: usize) {
        assert!(bit < self.nbits);

        let word = bit / 32;
        let b = bit % 32;
        let flag = 1 << b;

        self.data[word] = self.data[word] & !flag;
    }

    #[inline]
    pub fn distinct(&self, other: &BitVec) -> bool {
        assert_eq!(self.nbits, other.nbits);

        for i in 0..self.data.len() {
            let my_val = self.data[i];
            let other_val = other.data[i];

            if my_val & other_val != 0 {
                return false;
            }
        }

        true
    }

    #[inline]
    pub fn overlap(&self, other: &BitVec) -> bool {
        assert_eq!(self.nbits, other.nbits);

        for i in 0..self.data.len() {
            let my_val = self.data[i];
            let other_val = other.data[i];

            if my_val & other_val == 0 {
                return false;
            }
        }

        true
    }

    #[inline]
    pub fn all_overlap(&self, other: &BitVec) -> bool {
        assert_eq!(self.nbits, other.nbits);

        for i in 0..self.data.len() {
            let my_val = self.data[i];
            let other_val = other.data[i];

            if my_val & other_val != my_val {
                return false;
            }
        }

        true
    }
}

#[inline]
fn u32s(bits: usize) -> usize {
    if bits % 32 == 0 {
        bits / 32
    } else {
        bits / 32 + 1
    }
}