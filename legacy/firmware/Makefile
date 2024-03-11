APPVER = 1.8.0

NAME  = cerberus

SECP256K1_ZKP ?= 1


ifeq ($(SECP256K1_ZKP),1)
CFLAGS   += -DUSE_SECP256K1_ZKP
CFLAGS   += -DUSE_SECP256K1_ZKP_ECDSA
ifeq ($(EMULATOR),1)
CFLAGS += -DSECP256K1_CONTEXT_SIZE=208
else
CFLAGS += -DSECP256K1_CONTEXT_SIZE=180
OBJS += field_10x26_arm.o
endif
ZKP_CFLAGS = \
	-DUSE_ASM_ARM \
	-DUSE_EXTERNAL_ASM \
	-DUSE_EXTERNAL_DEFAULT_CALLBACKS \
	-DECMULT_GEN_PREC_BITS=4 \
	-DECMULT_WINDOW_SIZE=8 \
	-DENABLE_MODULE_GENERATOR \
	-DENABLE_MODULE_RECOVERY \
	-DENABLE_MODULE_SCHNORRSIG \
	-DENABLE_MODULE_EXTRAKEYS

OBJS += secp256k1-zkp.o
OBJS += precomputed_ecmult.o
OBJS += precomputed_ecmult_gen.o
OBJS += ../vendor/cerberus-crypto/zkp_bip340.o
OBJS += ../vendor/cerberus-crypto/zkp_context.o
OBJS += ../vendor/cerberus-crypto/zkp_ecdsa.o
endif

ifeq ($(EMULATOR),1)
OBJS += udp.o
else
OBJS += usb.o
OBJS += bl_check.o
OBJS += otp.o
OBJS += header.o
OBJS += startup.o
endif

OBJS += messages.o
OBJS += config.o
OBJS += cerberus.o
OBJS += pinmatrix.o
OBJS += fsm.o
OBJS += coins.o
OBJS += coin_info.o
OBJS += transaction.o
OBJS += protect.o
OBJS += layout2.o
OBJS += recovery.o
OBJS += reset.o
OBJS += signing.o
OBJS += crypto.o

ifneq ($(BITCOIN_ONLY),1)
OBJS += u2f.o
OBJS += ethereum.o
OBJS += ethereum_definitions.o
OBJS += ethereum_networks.o
OBJS += ethereum_tokens.o
OBJS += nem2.o
OBJS += nem_mosaics.o
OBJS += stellar.o
endif

OBJS += debug.o

OBJS += ../vendor/cerberus-crypto/address.o
OBJS += ../vendor/cerberus-crypto/bignum.o
OBJS += ../vendor/cerberus-crypto/ecdsa.o
OBJS += ../vendor/cerberus-crypto/curves.o
OBJS += ../vendor/cerberus-crypto/secp256k1.o
OBJS += ../vendor/cerberus-crypto/nist256p1.o
OBJS += ../vendor/cerberus-crypto/hmac_drbg.o
OBJS += ../vendor/cerberus-crypto/rfc6979.o
OBJS += ../vendor/cerberus-crypto/rand.o
OBJS += ../vendor/cerberus-crypto/memzero.o

OBJS += ../vendor/cerberus-crypto/ed25519-donna/curve25519-donna-32bit.o
OBJS += ../vendor/cerberus-crypto/ed25519-donna/curve25519-donna-helpers.o
OBJS += ../vendor/cerberus-crypto/ed25519-donna/modm-donna-32bit.o
OBJS += ../vendor/cerberus-crypto/ed25519-donna/ed25519-donna-basepoint-table.o
OBJS += ../vendor/cerberus-crypto/ed25519-donna/ed25519-donna-32bit-tables.o
OBJS += ../vendor/cerberus-crypto/ed25519-donna/ed25519-donna-impl-base.o
OBJS += ../vendor/cerberus-crypto/ed25519-donna/ed25519.o
OBJS += ../vendor/cerberus-crypto/ed25519-donna/curve25519-donna-scalarmult-base.o
OBJS += ../vendor/cerberus-crypto/ed25519-donna/ed25519-sha3.o
OBJS += ../vendor/cerberus-crypto/ed25519-donna/ed25519-keccak.o

OBJS += ../vendor/cerberus-crypto/hmac.o
OBJS += ../vendor/cerberus-crypto/bip32.o
OBJS += ../vendor/cerberus-crypto/bip39.o
OBJS += ../vendor/cerberus-crypto/bip39_english.o
OBJS += ../vendor/cerberus-crypto/pbkdf2.o
OBJS += ../vendor/cerberus-crypto/base32.o
OBJS += ../vendor/cerberus-crypto/base58.o
OBJS += ../vendor/cerberus-crypto/segwit_addr.o

OBJS += ../vendor/cerberus-crypto/ripemd160.o
OBJS += ../vendor/cerberus-crypto/sha2.o
OBJS += ../vendor/cerberus-crypto/sha3.o
OBJS += ../vendor/cerberus-crypto/blake256.o
OBJS += ../vendor/cerberus-crypto/blake2b.o
OBJS += ../vendor/cerberus-crypto/blake2s.o
OBJS += ../vendor/cerberus-crypto/groestl.o
OBJS += ../vendor/cerberus-crypto/hasher.o

OBJS += ../vendor/cerberus-crypto/aes/aescrypt.o
OBJS += ../vendor/cerberus-crypto/aes/aeskey.o
OBJS += ../vendor/cerberus-crypto/aes/aestab.o
OBJS += ../vendor/cerberus-crypto/aes/aes_modes.o

OBJS += ../vendor/cerberus-crypto/chacha20poly1305/chacha20poly1305.o
OBJS += ../vendor/cerberus-crypto/chacha20poly1305/chacha_merged.o
OBJS += ../vendor/cerberus-crypto/chacha20poly1305/poly1305-donna.o
OBJS += ../vendor/cerberus-crypto/chacha20poly1305/rfc7539.o

OBJS += ../vendor/cerberus-crypto/nem.o

OBJS += ../vendor/QR-Code-generator/c/qrcodegen.o

OBJS += ../vendor/cerberus-storage/storage.o
OBJS += ../vendor/cerberus-storage/storage_utils.o
OBJS += ../vendor/cerberus-storage/norcow.o

OBJS += ../vendor/nanopb/pb_common.o
OBJS += ../vendor/nanopb/pb_decode.o
OBJS += ../vendor/nanopb/pb_encode.o

OBJS += protob/messages.pb.o
OBJS += protob/messages-bitcoin.pb.o
OBJS += protob/messages-common.pb.o
OBJS += protob/messages-crypto.pb.o
OBJS += protob/messages-debug.pb.o
OBJS += protob/messages-management.pb.o

ifneq ($(BITCOIN_ONLY),1)
OBJS += ../vendor/cerberus-crypto/cash_addr.o
OBJS += protob/messages-ethereum.pb.o
OBJS += protob/messages-ethereum-definitions.pb.o
OBJS += protob/messages-nem.pb.o
OBJS += protob/messages-stellar.pb.o
endif

OPTFLAGS ?= -Os

../vendor/cerberus-crypto/bip32.o: OPTFLAGS = -O3
../vendor/cerberus-crypto/bip39.o: OPTFLAGS = -O3
../vendor/cerberus-crypto/ecdsa.o: OPTFLAGS = -O3
../vendor/cerberus-crypto/sha2.o: OPTFLAGS = -O3
../vendor/cerberus-crypto/secp256k1.o: OPTFLAGS = -O3

include ../Makefile.include


CFLAGS += -Wno-sequence-point
CFLAGS += -I../vendor/nanopb -Iprotob -DPB_FIELD_16BIT=1 -DPB_ENCODE_ARRAYS_UNPACKED=1 -DPB_VALIDATE_UTF8=1
CFLAGS += -DSCM_REVISION='"$(shell git rev-parse HEAD | sed 's:\(..\):\\x\1:g')"'
CFLAGS += -DUSE_MONERO=0
CFLAGS += -DUSE_OPTIGA=0
ifneq ($(BITCOIN_ONLY),1)
CFLAGS += -DUSE_ETHEREUM=1
CFLAGS += -DUSE_NEM=1
MAKO_RENDER_FLAG =
else
CFLAGS += -DUSE_ETHEREUM=0
CFLAGS += -DUSE_NEM=0
MAKO_RENDER_FLAG = --bitcoin-only
endif

%:: %.mako defs
	@printf "  MAKO    $@\n"
	$(Q)$(PYTHON) ../vendor/cerberus-common/tools/cointool.py render $(MAKO_RENDER_FLAG) $@.mako

ifeq ($(BOOTLOADER_QA), 0)
bl_data.h: bl_data.py bootloader.dat
	@printf "  PYTHON  bl_data.py bootloader.dat\n"
	$(Q)$(PYTHON) bl_data.py bootloader.dat
else
bl_data.h: bl_data.py bootloader_qa.dat
	@printf "  PYTHON  bl_data.py bootloader_qa.dat\n"
	$(Q)$(PYTHON) bl_data.py bootloader_qa.dat
endif

header.o: version.h

clean::
	rm -f bl_data.h
	find -maxdepth 1 -name "*.mako" | sed 's/.mako$$//' | xargs rm -f
