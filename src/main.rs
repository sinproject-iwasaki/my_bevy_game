#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

extern crate my_bevy_game;

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    my_bevy_game::run();
}
