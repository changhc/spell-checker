use rand::Rng;
use siphasher::sip128::SipHasher;

pub struct BloomFilter {
    size: u64,
    values: Vec<bool>,
    num_hash: u32,
    hasher: SipHasher,
}

fn get_random_key() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut key: [u8; 16] = [0u8; 16];
    rng.fill::<[u8; 16]>(&mut key);
    key
}

fn create_hasher(key: &[u8; 16]) -> SipHasher {
    SipHasher::new_with_key(key)
}

impl BloomFilter {
    pub fn new(size: u64, num_hash: u32) -> BloomFilter {
        let key = get_random_key();
        let hasher = create_hasher(&key);
        BloomFilter {
            size: size,
            values: vec![false; size as usize],
            num_hash: num_hash,
            hasher: hasher,
        }
    }

    pub fn insert(&mut self, value: &String) {
        let h128 = self.hasher.hash(value.as_bytes()).as_u128();
        let h0 = (h128 & 0xffff_ffff_ffff_ffff) as u64;
        let h1 = (h128 >> 64) as u64;
        let mut hash = h0;
        for _ in 0..self.num_hash {
            let bit = (hash % self.size) as usize;
            self.values[bit] = true;
            hash = hash.wrapping_add(h1);
        }
    }

    pub fn not_exist(&self, value: &String) -> bool {
        let h128 = self.hasher.hash(value.as_bytes()).as_u128();
        let h0 = (h128 & 0xffff_ffff_ffff_ffff) as u64;
        let h1 = (h128 >> 64) as u64;
        let mut hash = h0;
        for _ in 0..self.num_hash {
            let bit = (hash % self.size) as usize;
            if self.values[bit] == true {
                return false;
            }
            hash = hash.wrapping_add(h1);
        }
        true
    }
}
