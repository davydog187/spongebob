# Maintainer: Dave Lucia <davelucianyc@gmail.com>
pkgname=spongebob
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')

build() {
  cargo build --release --locked --all-features
}

package() {
  install -Dm 755 ${startdir}/target/release/${pkgname} -t "${pkgdir}/usr/bin"
}
