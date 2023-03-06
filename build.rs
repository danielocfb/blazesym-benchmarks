use std::env;
use std::fs::File;
use std::io::copy;
use std::path::Path;

use xz2::read::XzDecoder;


/// Unpack an xz compressed file.
fn unpack_xz(src: &Path, dst: &Path) {
    let src_file = File::options().create(false).read(true).open(src).unwrap();
    let mut decoder = XzDecoder::new_multi_decoder(src_file);

    let mut dst_file = File::options()
        .create(true)
        .truncate(true)
        .read(false)
        .write(true)
        .open(dst)
        .unwrap();

    copy(&mut decoder, &mut dst_file).unwrap();
}

/// Unpack benchmark files.
fn unpack_bench_files(crate_root: &Path) {
    let src = Path::new(crate_root)
        .join("data")
        .join("vmlinux-5.17.12-100.fc34.x86_64.xz");
    println!("cargo:rerun-if-changed={}", src.display());

    let mut dst = src.clone();
    assert!(dst.set_extension(""));
    println!("cargo:rerun-if-changed={}", dst.display());

    unpack_xz(&src, &dst)
}

fn main() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    unpack_bench_files(crate_root)
}
