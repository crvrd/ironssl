extern crate rand;
extern crate num;

// Efficient modular exponentiation using square-multiply method
// returns (b^e) mod m
fn modexp(b: u64, e: u64, m: u64) -> u64
{
	let mut base = b;
	let mut exp = e;
	let modulus = m;
	let mut result = 1u64;

	while exp > 0
	{
		if exp % 2 == 1
		{
			result = (result * base) % modulus;
		}
		exp = exp >> 1;
		base = (base * base) % modulus;
	}

	return result;
}

// Miller-Rabin primality check, fixed at 40 rounds (because that's what a tl;dr of a StackOverflow answer told me)
// Returns true if the number if prime
fn miller_rabin(num : u64) -> bool
{
	// If num is even, we immediately know it's not prime
	if num % 2 == 0
	{
		return false;
	}

	// Calculate pre-reqs
	let mut s = 1u64;
	let mut t : u64	= (num-1) / 2;

	while t % 2 == 0
	{
		t /= 2;
		s += 1
	}

	t = t as u64;

	for _r in 0..40
	{
		// generate rand, no need for cryptographically secure
		let rand_temp = (rand::random::<f64>() * ((num-1) as f64)) + 1f64;
		let rand = rand_temp as u64;
		let mut y : u64 = modexp(rand, t, num);
		let mut prime : bool = false;

		if y == 1
		{
			prime = true;
		}

		for _i in 0..s
		{
			if y == num-1
			{
				prime = true;
				break;
			}
			else
			{
				y = modexp(y, 2, num);
			}
		}

		if prime == false
		{
			return false;
		}
	}

	return true

}

// Checks whether g is a primitive root of p
// returns true if g is the primitive root of p
fn is_primitive(p : u64, g : u64) -> bool
{
	// at this point we've already checked that p is prime
	//let mut totient = p - 1;

	let mut ret = true;

	if g > p
	{
		ret = false;
	}

	if num::integer::gcd(g,p) != 1
	{
		ret = false;
	}

	return ret;
}

// fn generate_prime()
// {
// 	println!("hello");
// }

// returns shared key
// checks if things are valid, then returns b^a_sec mod p
fn compute_shared_key(p : u64, g : u64, a_sec : u64, b : u64) -> u64
{
	// check if p is prime and that g is a primitive root of p
	// if both conditions match, do the stuff
	if miller_rabin(p) && is_primitive(p, g)
	{
		// pick a random secret code
		return modexp(b, a_sec, p);
	}

	return 0;
}

fn main()
{
	//generate_prime();

	// pre-shared info
	let g = 5;
	let p = 23;

	// alice's info
	let a_key = 6;
	let a_exp = modexp(g, a_key, p); // sends to bob

	// bob's info
	let b_key = 15;
	let b_exp = modexp(g, b_key, p);// sends to alice

	let shared_1 = compute_shared_key(p, g, a_key, b_exp);
	let shared_2 = compute_shared_key(p, g, b_key, a_exp);

	println!("{} {}", shared_1, shared_2);
	
}