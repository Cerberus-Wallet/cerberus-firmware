NAME  = bootloader-unaligned

OBJS += startup.o
OBJS += bootloader.o
OBJS += usb.o

# Overrides from libcerberus.
# Objects ending with 'small.o' are built with -Os to make the bootloader smaller.
CFLAGS += -DFONT_SKIP_FIXED=1
OBJS += ../buttons.small.o
OBJS += ../common.small.o
OBJS += ../flash.small.o
OBJS += ../fw_signatures.small.o
OBJS += ../gen/bitmaps.small.o
OBJS += ../gen/fonts.small.o
OBJS += ../layout.small.o
OBJS += ../memory.small.o
OBJS += ../oled.small.o
OBJS += ../random_delays.small.o
OBJS += ../rng.small.o
OBJS += ../setup.small.o
OBJS += ../supervise.small.o
OBJS += ../timer.small.o
OBJS += ../usb21_standard.small.o
OBJS += ../usb_standard.small.o
OBJS += ../util.small.o
OBJS += ../webusb.small.o
OBJS += ../winusb.small.o

# Overrides from cerberus-crypto
CFLAGS += -DUSE_PRECOMPUTED_CP=0
CFLAGS += -DUSE_PRECOMPUTED_IV=0
OBJS += ../vendor/cerberus-crypto/bignum.small.o
OBJS += ../vendor/cerberus-crypto/ecdsa.small.o
OBJS += ../vendor/cerberus-crypto/hmac_drbg.small.o
OBJS += ../vendor/cerberus-crypto/memzero.small.o
OBJS += ../vendor/cerberus-crypto/secp256k1.small.o
OBJS += ../vendor/cerberus-crypto/sha2.small.o

OPTFLAGS ?= -Os

include ../Makefile.include

# Remove libcerberus from linking since we specified the small versions
LDLIBS := $(filter-out -lcerberus,$(LDLIBS))

bootloader.bin: $(NAME).bin
	./firmware_align.py $< $@

all: bootloader.bin
