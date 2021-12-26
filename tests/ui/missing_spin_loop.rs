// run-rustfix
#![warn(clippy::missing_spin_loop)]

use core::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let b = AtomicBool::new(true);
    // This should lint
    while b.load(Ordering::Acquire) {}

    // This is OK, as the body is not empty
    while b.load(Ordering::Acquire) {
        std::hint::spin_loop()
    }
    // TODO: also match on compare_exchange(_weak), but that could be
    // loop+match or while let
}
