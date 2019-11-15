#![feature(test)]

extern crate gol;
extern crate test;

#[bench]
fn universe_tick(b: &mut test::Bencher) {
    let mut universe = gol::Universe::new();

    b.iter(|| {
        universe.tick();
    });
}
