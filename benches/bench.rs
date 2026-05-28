use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_minimap2(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-minimap2");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let ref_fa = manifest.join("tests/golden/ref.fa");
    let reads = manifest.join("tests/golden/reads.fa");
    c.bench_function("rsomics-minimap2 golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .args([ref_fa.to_str().unwrap(), reads.to_str().unwrap()])
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_minimap2);
criterion_main!(benches);
