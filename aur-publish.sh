#!/bin/zsh

mv ./PKGBUILD /aur/PKGBUILD
makepkg --printsrcinfo > .SRCINFO
git commit -m "publish v${grep -oP '(?<=pkgver=).*(?=)' PKGBUILD}" .SRCINFO PKGBUILD
git push