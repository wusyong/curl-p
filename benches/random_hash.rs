#[macro_use]
extern crate criterion;
extern crate curl;
extern crate rand;

use criterion::Criterion;
use curl::{stateless::digest, Curl};
use rand::{thread_rng, Rng};

fn basic_curl(trits: &[i8]) {
    let mut curl = Curl::default();
    let mut hash = vec![0; 243];
    curl.digest(&trits, &mut hash);
}

fn stateless_curl(trits: &[i8]) {
    let _ = digest(&trits, 81);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut trits = [0; 8019];
    for trit in trits.iter_mut() {
        *trit = rng.gen_range(-1, 2);
    }
    c.bench_function("Curl on 8019 trits", move |b| b.iter(|| basic_curl(&trits)));
    c.bench_function("Stateless on 8019 trits", move |b| {
        b.iter(|| stateless_curl(&trits))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
