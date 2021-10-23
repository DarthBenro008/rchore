# This is an example PKGBUILD file. Use this as a start to creating your own,
# and remove these comments. For more information, see 'man PKGBUILD'.
# NOTE: Please fill out the license field for your package! If it is unknown,
# then please put 'unknown'.

# The following guidelines are specific to BZR, GIT, HG and SVN packages.
# Other VCS sources are not natively supported by makepkg yet.

# Maintainer: pspiagicw <pspiagicw@gmail.com>
pkgname='rchore-bin'
pkgver=0.1.0
pkgrel=1
pkgdesc="A feature packed Google Tasks CLI written in Rust"
arch=('x86_64')
url="https://lib.rs/crates/rchore"
license=('MIT')
groups=()
depends=('glibc')
makedepends=() # 'bzr', 'git', 'mercurial' or 'subversion'
provides=("${pkgname%-bin}")
conflicts=("${pkgname%-bin}")
replaces=()
backup=()
options=()
install=
source=("https://github.com/DarthBenro008/rchore/releases/latest/download/rchore-linux"
	"https://raw.githubusercontent.com/DarthBenro008/rchore/master/README.md")
noextract=()
md5sums=('SKIP' 'SKIP')


package() {
    install -Dm755 "rchore-linux" "$pkgdir/usr/bin/rchore"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
