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

#include "librust.h"
#include "librust_fonts.h"
#include "py/runtime.h"

#if MICROPY_PY_CERBERUSUI2
MP_REGISTER_MODULE(MP_QSTR_cerberusui2, mp_module_cerberusui2);
#endif

#if MICROPY_PY_CERBERUSPROTO
MP_REGISTER_MODULE(MP_QSTR_cerberusproto, mp_module_cerberusproto);
#endif

#if MICROPY_PY_CERBERUSTRANSLATE
MP_REGISTER_MODULE(MP_QSTR_cerberustranslate, mp_module_cerberustranslate);
#endif
