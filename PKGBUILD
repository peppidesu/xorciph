# Maintainer: Pepijn Bakker (peppidesu)
#
# This PKGBUILD was generated by `cargo aur`: https://crates.io/crates/cargo-aur

pkgname=xorciph-bin
pkgver=0.3.1
pkgrel=1
pkgdesc="XOR cipher CLI written in Rust."
url="https://github.com/peppidesu/xorciph"
license=("APACHE")
arch=("x86_64")
provides=("xorciph")
conflicts=("xorciph")
source=("https://github.com/peppidesu/xorciph/releases/download/$pkgver/xorciph-$pkgver-x86_64.tar.gz")
sha256sums=("696bee7b1be86e47b1a66c14bcad6f7bdafc70d834ead62d05a9491f43121ca8")

package() {
    install -Dm755 xorciph -t "$pkgdir/usr/bin"
}
