# Horizonto — konstruado kaj la loka kvalita pordego.
# Postulas Rust 1.100.0 kun clippy, rustfmt kaj la du `no_std` celojn,
# kaj `just`. Miri postulas noktan ilĉenon; `cargo-deny` nur en DI.
#
# Q01: `just kontroli` estas la loka pordego, sen Kani kaj sen reto.

default: kontroli

# La plena loka pordego laŭ la ordo de HORIZONTO.md §6.2.
kontroli:
    bash tools/versio.sh --kontroli
    bash tools/sen-cina.sh --kontroli
    cargo fmt --all -- --check
    cargo clippy --workspace --release -- -D warnings
    cargo clippy -p libhorizonto --release --target x86_64-unknown-none -- -D warnings
    cargo clippy -p libhorizonto --release --target aarch64-unknown-none -- -D warnings
    cargo test --workspace
    cargo build --workspace --release

# Krucaj konstruloj por ambaŭ `no_std` celoj.
konstrui:
    cargo build --workspace --release
    cargo build -p libhorizonto --release --target x86_64-unknown-none
    cargo build -p libhorizonto --release --target aarch64-unknown-none

# La publika dokumentaro sen mankantaj priskriboj (§6.5 punkto 5).
dokumentaro:
    RUSTDOCFLAGS='-D missing-docs' cargo doc --no-deps -p libhorizonto

# Miri super la unu-testoj de `libhorizonto` (§6.3 Q07).
miri:
    cargo +nightly miri test -p libhorizonto

# Permesila kontrolo laŭ `deny.toml` (§6.3 Q04).
permesiloj:
    cargo deny check

# La lingva kontrolo de la akcepto.
sen-cina:
    bash tools/sen-cina.sh --kontroli

# La pureco de libhorizonto: sen glitkomo, unsafe aŭ alloc.
sen-glitkomo:
    bash tools/sen-glitkomo.sh --kontroli

versio:
    bash tools/versio.sh --kontroli
