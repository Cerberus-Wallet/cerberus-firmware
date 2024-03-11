#!/bin/sh

set -e

cd $(dirname $0)

VERSION="2"

install -D -m 0644 ./cerberus.rules    ./lib/udev/rules.d/52-cerberus-extension.rules

NAME=cerberus-udev

tar cfj $NAME-$VERSION.tar.bz2 ./lib

for TYPE in "deb" "rpm"; do
	fpm \
		-s tar \
		-t $TYPE \
		-a all \
		-n $NAME \
		-v $VERSION \
		--license "LGPL-3.0" \
		--vendor "SatoshiLabs" \
		--description "Udev rules for Cerberus" \
		--maintainer "SatoshiLabs <stick@satoshilabs.com>" \
		--url "https://cerberus.uraanai.com/" \
		--category "Productivity/Security" \
		$NAME-$VERSION.tar.bz2
done

rm $NAME-$VERSION.tar.bz2
rm -rf  ./lib
