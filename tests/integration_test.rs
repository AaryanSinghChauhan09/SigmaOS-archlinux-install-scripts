// Integration test for SigmaOS-archlinux-install-scripts
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use archlinux_install_scripts as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
