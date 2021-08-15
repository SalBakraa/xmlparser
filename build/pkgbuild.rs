use std::fs;
use std::io::Write;

use toml::Value;

pub fn write_pkgbuild() -> Result<(), std::io::Error> {
    let toml = fs::read_to_string("Cargo.toml").unwrap();
    let toml = match toml.parse::<Value>().unwrap() {
        Value::Table(mut table) => table.remove("package").unwrap(),
        _ => panic!("Cargo.toml didn't return a table"),
    };

    let mut pkgbuild = fs::File::create("pkg/aur/PKGBUILD").unwrap();

    toml["authors"].as_array().unwrap().iter()
        .map(|a| a.as_str().unwrap())
        .for_each(|a| { let _ = writeln!(pkgbuild, "## Maintainer: {}", a); });

    writeln!(pkgbuild, "#")?;
    writeln!(pkgbuild, "# File was auto generated through build.rs")?;
    writeln!(pkgbuild)?;

    if let Some(name) = toml.get("name") {
        writeln!(pkgbuild, "pkgname='{}'", name.as_str().unwrap())?;
    }

    if let Some(version) = toml.get("version") {
        writeln!(pkgbuild, "pkgver='{}'", version.as_str().unwrap())?;
    }

    writeln!(pkgbuild, "pkgrel=1")?;

    if let Some(description) = toml.get("description") {
        writeln!(pkgbuild, "pkgdesc='{}'", description.as_str().unwrap())?;
    }

    if let Some(repository) = toml.get("repository") {
        writeln!(pkgbuild, "url='{}'", repository.as_str().unwrap())?;
    }

    writeln!(pkgbuild, "arch=('any')")?;

    // Wait for crate that converts spdx licenses to aur licenses
    if let Some(license) = toml.get("license") {
        writeln!(pkgbuild, "license=('{}')", license.as_str().unwrap())?;
    }

    writeln!(pkgbuild, "provides=(\"$pkgname\")")?;
    writeln!(pkgbuild, "conflicts=(\"$pkgname\")")?;
    writeln!(pkgbuild, "source=(\"$pkgname-$pkgver::git+$url.git\")")?;
    writeln!(pkgbuild, "depends=('libxml2')")?;
    writeln!(pkgbuild, "makedepends=('cargo' 'git')")?;
    writeln!(pkgbuild, "sha256sums=('SKIP')")?;

    write!(pkgbuild, r#"
prepare() {{
    cd "$srcdir/$pkgname-$pkgver"

    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}}
"#)?;


    write!(pkgbuild, r#"
build() {{
    cd "$srcdir/$pkgname-$pkgver"

    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release
}}
"#)?;

    write!(pkgbuild, r#"
check() {{
    cd "$srcdir/$pkgname-$pkgver"

    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=test
    cargo test --frozen --release
}}
"#)?;

    write!(pkgbuild, r#"
package() {{
    cd "$srcdir/$pkgname-$pkgver"
    local OUT_DIR="$(< target/out_dir)"

    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

    install -Dm644 "$OUT_DIR/_$pkgname" "$pkgdir/usr/share/zsh/site-functions/_$pkgname"
    install -Dm644 "$OUT_DIR/$pkgname.bash" "$pkgdir/usr/share/bash-completion/completions/$pkgname"
    install -Dm644 "$OUT_DIR/$pkgname.fish" "$pkgdir/usr/share/fish/vendor_completions.d/$pkgname.fish"

    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}}
"#)
}
