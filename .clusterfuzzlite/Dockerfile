FROM gcr.io/oss-fuzz-base/base-builder
RUN apt-get update && apt-get install -y make autoconf automake libtool \
  pkg-config
COPY . $SRC/cerberus-firmware
WORKDIR $SRC/cerberus-firmware/crypto
COPY .clusterfuzzlite/build.sh $SRC/
