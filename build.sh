#! /bin/sh

CARGO=cargo
TARGET=x86_64-unknown-linux-gnu
APP=bootstrap
DISTDIR=dist
DISTFILE=lambda.zip

set -e

$CARGO build --release --target $TARGET

mkdir -p $DISTDIR
cp target/$TARGET/release/$APP $DISTDIR
cd $DISTDIR
zip $DISTFILE ./$APP

echo "TODO: upload to AWS"

#cd -
#rm -rf $DISTDIR
