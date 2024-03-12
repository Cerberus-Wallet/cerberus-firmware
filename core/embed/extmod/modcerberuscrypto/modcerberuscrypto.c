/*
 * This file is part of the Cerberus project, https://cerberus.uraanai.com/
 *
 * Copyright (c) SatoshiLabs
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "common.h"

#include "py/runtime.h"

#include CERBERUS_BOARD

#if MICROPY_PY_CERBERUSCRYPTO

static mp_obj_t ui_wait_callback = mp_const_none;

static void wrapped_ui_wait_callback(uint32_t current, uint32_t total) {
  if (mp_obj_is_callable(ui_wait_callback)) {
    mp_call_function_2_protected(ui_wait_callback, mp_obj_new_int(current),
                                 mp_obj_new_int(total));
  }
}

#include "modcerberuscrypto-aes.h"
#include "modcerberuscrypto-bech32.h"
#include "modcerberuscrypto-bip32.h"
#ifdef USE_SECP256K1_ZKP
#include "modcerberuscrypto-bip340.h"
#endif
#include "modcerberuscrypto-bip39.h"
#include "modcerberuscrypto-blake256.h"
#include "modcerberuscrypto-blake2b.h"
#include "modcerberuscrypto-blake2s.h"
#include "modcerberuscrypto-chacha20poly1305.h"
#include "modcerberuscrypto-crc.h"
#include "modcerberuscrypto-curve25519.h"
#include "modcerberuscrypto-ed25519.h"
#include "modcerberuscrypto-groestl.h"
#include "modcerberuscrypto-hmac.h"
#include "modcerberuscrypto-nist256p1.h"
#include "modcerberuscrypto-pbkdf2.h"
#include "modcerberuscrypto-random.h"
#include "modcerberuscrypto-ripemd160.h"
#include "modcerberuscrypto-secp256k1.h"
#include "modcerberuscrypto-sha1.h"
#include "modcerberuscrypto-sha256.h"
#include "modcerberuscrypto-sha3-256.h"
#include "modcerberuscrypto-sha3-512.h"
#include "modcerberuscrypto-sha512.h"
#include "modcerberuscrypto-shamir.h"
#include "modcerberuscrypto-slip39.h"
#ifdef USE_OPTIGA
#include "modcerberuscrypto-optiga.h"
#endif
#if !BITCOIN_ONLY
#include "modcerberuscrypto-cardano.h"
#include "modcerberuscrypto-monero.h"
#include "modcerberuscrypto-nem.h"
#endif

STATIC const mp_rom_map_elem_t mp_module_cerberuscrypto_globals_table[] = {
    {MP_ROM_QSTR(MP_QSTR___name__), MP_ROM_QSTR(MP_QSTR_cerberuscrypto)},
    {MP_ROM_QSTR(MP_QSTR_aes), MP_ROM_PTR(&mod_cerberuscrypto_AES_type)},
    {MP_ROM_QSTR(MP_QSTR_bech32), MP_ROM_PTR(&mod_cerberuscrypto_bech32_module)},
    {MP_ROM_QSTR(MP_QSTR_bip32), MP_ROM_PTR(&mod_cerberuscrypto_bip32_module)},
    {MP_ROM_QSTR(MP_QSTR_bip39), MP_ROM_PTR(&mod_cerberuscrypto_bip39_module)},
    {MP_ROM_QSTR(MP_QSTR_blake256),
     MP_ROM_PTR(&mod_cerberuscrypto_Blake256_type)},
    {MP_ROM_QSTR(MP_QSTR_blake2b), MP_ROM_PTR(&mod_cerberuscrypto_Blake2b_type)},
    {MP_ROM_QSTR(MP_QSTR_blake2s), MP_ROM_PTR(&mod_cerberuscrypto_Blake2s_type)},
#if !BITCOIN_ONLY
    {MP_ROM_QSTR(MP_QSTR_cardano),
     MP_ROM_PTR(&mod_cerberuscrypto_cardano_module)},
#endif
    {MP_ROM_QSTR(MP_QSTR_chacha20poly1305),
     MP_ROM_PTR(&mod_cerberuscrypto_ChaCha20Poly1305_type)},
    {MP_ROM_QSTR(MP_QSTR_crc), MP_ROM_PTR(&mod_cerberuscrypto_crc_module)},
    {MP_ROM_QSTR(MP_QSTR_curve25519),
     MP_ROM_PTR(&mod_cerberuscrypto_curve25519_module)},
    {MP_ROM_QSTR(MP_QSTR_ed25519),
     MP_ROM_PTR(&mod_cerberuscrypto_ed25519_module)},
#if !BITCOIN_ONLY
    {MP_ROM_QSTR(MP_QSTR_monero), MP_ROM_PTR(&mod_cerberuscrypto_monero_module)},
#endif
    {MP_ROM_QSTR(MP_QSTR_nist256p1),
     MP_ROM_PTR(&mod_cerberuscrypto_nist256p1_module)},
    {MP_ROM_QSTR(MP_QSTR_groestl512),
     MP_ROM_PTR(&mod_cerberuscrypto_Groestl512_type)},
    {MP_ROM_QSTR(MP_QSTR_hmac), MP_ROM_PTR(&mod_cerberuscrypto_Hmac_type)},
#if !BITCOIN_ONLY
    {MP_ROM_QSTR(MP_QSTR_nem), MP_ROM_PTR(&mod_cerberuscrypto_nem_module)},
#endif
    {MP_ROM_QSTR(MP_QSTR_pbkdf2), MP_ROM_PTR(&mod_cerberuscrypto_Pbkdf2_type)},
    {MP_ROM_QSTR(MP_QSTR_random), MP_ROM_PTR(&mod_cerberuscrypto_random_module)},
    {MP_ROM_QSTR(MP_QSTR_ripemd160),
     MP_ROM_PTR(&mod_cerberuscrypto_Ripemd160_type)},
    {MP_ROM_QSTR(MP_QSTR_secp256k1),
     MP_ROM_PTR(&mod_cerberuscrypto_secp256k1_module)},
#if USE_SECP256K1_ZKP
    {MP_ROM_QSTR(MP_QSTR_bip340), MP_ROM_PTR(&mod_cerberuscrypto_bip340_module)},
#endif
    {MP_ROM_QSTR(MP_QSTR_sha1), MP_ROM_PTR(&mod_cerberuscrypto_Sha1_type)},
    {MP_ROM_QSTR(MP_QSTR_sha256), MP_ROM_PTR(&mod_cerberuscrypto_Sha256_type)},
    {MP_ROM_QSTR(MP_QSTR_sha512), MP_ROM_PTR(&mod_cerberuscrypto_Sha512_type)},
    {MP_ROM_QSTR(MP_QSTR_sha3_256),
     MP_ROM_PTR(&mod_cerberuscrypto_Sha3_256_type)},
    {MP_ROM_QSTR(MP_QSTR_sha3_512),
     MP_ROM_PTR(&mod_cerberuscrypto_Sha3_512_type)},
    {MP_ROM_QSTR(MP_QSTR_shamir), MP_ROM_PTR(&mod_cerberuscrypto_shamir_module)},
    {MP_ROM_QSTR(MP_QSTR_slip39), MP_ROM_PTR(&mod_cerberuscrypto_slip39_module)},
#if USE_OPTIGA
    {MP_ROM_QSTR(MP_QSTR_optiga), MP_ROM_PTR(&mod_cerberuscrypto_optiga_module)},
#endif
};
STATIC MP_DEFINE_CONST_DICT(mp_module_cerberuscrypto_globals,
                            mp_module_cerberuscrypto_globals_table);

const mp_obj_module_t mp_module_cerberuscrypto = {
    .base = {&mp_type_module},
    .globals = (mp_obj_dict_t *)&mp_module_cerberuscrypto_globals,
};

MP_REGISTER_MODULE(MP_QSTR_cerberuscrypto, mp_module_cerberuscrypto);

#ifdef USE_SECP256K1_ZKP
void secp256k1_default_illegal_callback_fn(const char *str, void *data) {
  (void)data;
  mp_raise_ValueError(str);
  return;
}

void secp256k1_default_error_callback_fn(const char *str, void *data) {
  (void)data;
  __fatal_error(NULL, str, __FILE__, __LINE__, __func__);
  return;
}
#endif

#endif  // MICROPY_PY_CERBERUSCRYPTO
