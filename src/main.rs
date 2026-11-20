//! SigmaOS-updatemanager2 — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_updatemanager2_lib::{Component, ComponentLifecycle};

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-updatemanager2: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-updatemanager2: ok");
}
