use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use std::collections::BTreeMap;
use std::time::Instant;
use xor_name::{Prefix, XorName};

// Generates random ed25519 keypairs to get a keypair for every prefix.
// Reports how long it takes to find keypairs for the whole space.

fn main() {
    let start = Instant::now();
    let mut prev = start;
    // initialize prefix lengths with the number of keys matching
    const max_prefix: usize = 30;
    let mut key_count_for_prefix_len = [0usize; max_prefix];
    // create a cache for the keys
    let mut key_cache: BTreeMap<Prefix, Keypair> = BTreeMap::new();
    let mut csprng = OsRng{};
    // generate a lot of random keys
    for i in 0..u64::MAX {
        let keypair: Keypair = Keypair::generate(&mut csprng);
        let xorname = XorName(keypair.public.to_bytes());
        let kpb = keypair.to_bytes();
        // cache this keypair for each new prefix it matches
        for i in 1..max_prefix {
            let kp = Keypair::from_bytes(&kpb).unwrap();
            let p = Prefix::new(i, xorname);
            match key_cache.get(&p) {
                Some(existing_prefix) => {
                    /* nothing to do */
                }
                None => {
                    key_cache.insert(p, kp);
                    key_count_for_prefix_len[i] = key_count_for_prefix_len[i] + 1;
                }
            }
        }
        if prev.elapsed().as_secs() > 5 {
            report(key_count_for_prefix_len, start);
            prev = Instant::now();
        }
    }
}

fn report(counts: [usize; 30], start: Instant) {
    println!("\nState of key cache after {:?} seconds", start.elapsed().as_secs());
    for i in 1..30 {
        let total: usize = 1<<i;
        let count: f64 = counts[i] as f64;
        let filled = count / (total as f64) * 100.0; // perecent
        println!("{:?} / {:?} keys for prefix len {:?}, covering {:?}% of the space", counts[i], total, i, filled);
    }
}
