static LOWER_MASK: u64 = 0x00000000FFFFFFFF;
static UPPER_MASK: u64 = 0xFFFFFFFF00000000;
static N: usize = 312;
static U: u64 = 22;
static S: u64 = 34;
static B: u64 = 0x12522789FC82583A;
static T: u64 = 30;
static C: u64 = 0x043640BFCABDEE38;
static L: u64 = 36;
static F: u64 = 1812433253;
static M: usize = 397;

pub struct MTRand {
	state: [u64; 312], // Here 312 is N
	index: usize,
}

impl MTRand {

	pub fn seed(seed: u64) -> MTRand {
		let mut new: MTRand = MTRand {state: [0; 312], index: N}; // Here 312 is N
		new.state[0] = seed;
		for i in 1..N {
			let old_state: u64 = new.state[i-1];
			new.state[i] = LOWER_MASK & ((F *
				old_state ^ (old_state >> 30)) + i as u64);
		}
		new
	}

	fn twist(&mut self) {
		for i in 0..N-1 {
			let x: u64 = (self.state[i] & UPPER_MASK)
				+ (self.state[(i + 1) % N] & LOWER_MASK);

			self.state[i] = self.state[(i + M) % N] ^ (x >> 1);

			if x & 2 != 0 {
				let temp = self.state[i];
				self.state[i] = temp ^ 0x9908b0df9908b0df;
			}
		}
		self.index = 0;
	}

	pub fn next(&mut self) -> u64 {
		if self.index >= N {
			self.twist();
		}

		let mut x = self.state[self.index];

		x = x ^ (x >> U);
		x = x ^ ((x << S) & B);
		x = x ^ ((x << T) & C);
		x = x ^ (x >> L);

		self.index = self.index + 1;

		x
	}

	pub fn next_range(&mut self, max: u64) -> u64 {
		let number = self.next();
		number % max
	}
}

mod test {

	use MTRand;
	
	#[test]
	fn value_test() {
		let mut rand1 = MTRand::seed(5051);
		let mut rand2 = MTRand::seed(5050);

		assert!(rand1.next() != rand2.next());
		assert!(rand1.next() != rand2.next());
		assert!(rand1.next() != rand2.next());
		assert!(rand1.next() != rand2.next());
		assert!(rand1.next() != rand2.next());
	}

	#[test]
	fn with_max_test(){
		let mut rand = MTRand::seed(1337);
		let max = 10;

		assert!(rand.next_range(max) < max);
		assert!(rand.next_range(max) < max);
		assert!(rand.next_range(max) < max);
		assert!(rand.next_range(max) < max);
	}
}