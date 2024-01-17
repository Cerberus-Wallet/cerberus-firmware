/*
 * This file is part of the Trezor project, https://trezor.io/
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

#include "gdc_core.h"
#include "gdc_dma2d.h"

#include <string.h>

#if USE_DMA2D
#include "dma2d.h"
#endif

static void gdc_rgb565_release(gdc_t* gdc) {
  /* gdc_bitmap_t* bitmap = (gdc_bitmap_t*) gdc;

  if (bitmap->release != NULL) {
      bitmap->release(bitmap->context);
  }*/
}

static gdc_bitmap_t* gdc_rgb565_get_bitmap(gdc_t* gdc) {
  return (gdc_bitmap_t*)gdc;
}

/*static*/ bool gdc_rgb565_fill(gdc_t* gdc, dma2d_params_t* params) {
  dma2d_params_t p = *params;

  uint16_t* dst_ptr = (uint16_t*)p.dst_row + p.dst_x;

  while (p.height-- > 0) {
    for (int x = 0; x < p.width; x++) {
      dst_ptr[x] = p.srca_fg;
    }
    dst_ptr += p.dst_stride / sizeof(*dst_ptr);
  }

  return true;
}

/*static*/ bool gdc_rgb565_copy_mono4(gdc_t* gdc, dma2d_params_t* params) {
  dma2d_params_t p = *params;

  const gdc_color16_t* gradient = gdc_color16_gradient_a4(p.srca_fg, p.srca_bg);

  uint16_t* dst_ptr = (uint16_t*)p.dst_row + p.dst_x;
  uint8_t* srca_row = (uint8_t*)p.srca_row;

  while (p.height-- > 0) {
    for (int x = 0; x < p.width; x++) {
      uint8_t fg_data = srca_row[(x + p.srca_x) / 2];
      uint8_t fg_lum = (x + p.srca_x) & 1 ? fg_data >> 4 : fg_data & 0xF;
      dst_ptr[x] = gradient[fg_lum];
    }
    dst_ptr += p.dst_stride / sizeof(*dst_ptr);
    srca_row += p.srca_stride / sizeof(*srca_row);
  }

  return true;
}

/*static*/ bool gdc_rgb565_copy_rgb565(gdc_t* gdc, dma2d_params_t* params) {
#if defined(USE_DMA2D) && !defined(TREZOR_EMULATOR)
  if (!params->cpu_only) {
    return dma2d_rgb565_copy_rgb565(gdc, params);
  } else
#endif
  {
    dma2d_params_t p = *params;

    uint16_t* dst_ptr = (uint16_t*)p.dst_row + p.dst_x;
    uint16_t* srca_ptr = (uint16_t*)p.srca_row + p.srca_x;

    while (p.height-- > 0) {
      for (int x = 0; x < p.width; x++) {
        dst_ptr[x] = srca_ptr[x];
      }
      dst_ptr += p.dst_stride / sizeof(*dst_ptr);
      srca_ptr += p.srca_stride / sizeof(*srca_ptr);
    }

    return true;
  }
}

static bool gdc_rgb565_blend_mono4_mono4(gdc_t* gdc, dma2d_params_t* params) {
  dma2d_params_t p = *params;

  const gdc_color16_t* gradient = gdc_color16_gradient_a4(p.srcb_fg, p.srcb_bg);

  uint16_t* dst_ptr = (uint16_t*)p.dst_row + p.dst_x;
  uint8_t* srca_row = (uint8_t*)p.srca_row;
  uint8_t* srcb_row = (uint8_t*)p.srcb_row;

  while (p.height-- > 0) {
    for (int x = 0; x < p.width; x++) {
      uint8_t fg_data = srca_row[(x + p.srca_x) / 2];
      uint8_t fg_alpha = (x + p.srca_x) & 1 ? fg_data >> 4 : fg_data & 0x0F;
      uint8_t bg_data = srcb_row[(x + p.srcb_x) / 2];
      uint8_t bg_lum = (x + p.srcb_x) & 1 ? bg_data >> 4 : bg_data & 0x0F;
      dst_ptr[x] = gdc_color16_blend_a4(
          p.srca_fg, gdc_color16_to_color(gradient[bg_lum]), fg_alpha);
    }
    dst_ptr += p.dst_stride / sizeof(*dst_ptr);
    srca_row += p.srca_stride / sizeof(*srca_row);
    srcb_row += p.srcb_stride / sizeof(*srcb_row);
  }

  return true;
}

/*static*/ bool gdc_rgb565_blend_mono4_rgb565(gdc_t* gdc,
                                              dma2d_params_t* params) {
  dma2d_params_t p = *params;

  uint16_t* dst_ptr = (uint16_t*)p.dst_row + p.dst_x;
  uint8_t* srca_row = (uint8_t*)p.srca_row;
  uint16_t* srcb_ptr = (uint16_t*)p.srcb_row + p.srcb_x;

  while (p.height-- > 0) {
    for (int x = 0; x < p.width; x++) {
      uint8_t fg_data = srca_row[(x + p.srca_x) / 2];
      uint8_t fg_alpha = (x + p.srca_x) & 1 ? fg_data >> 4 : fg_data & 0x0F;
      dst_ptr[x] = gdc_color16_blend_a4(
          p.srca_fg, gdc_color16_to_color(srcb_ptr[x]), fg_alpha);
    }
    dst_ptr += p.dst_stride / sizeof(*dst_ptr);
    srca_row += p.srca_stride / sizeof(*srca_row);
    srcb_ptr += p.srcb_stride / sizeof(*srcb_ptr);
  }

  return true;
}

gdc_bitmap_t gdc_bitmap_rgb565(void* data_ptr, size_t stride, gdc_size_t size,
                               uint8_t attrs) {
  static const gdc_vmt_t gdc_rgb565 = {
    .release = gdc_rgb565_release,
    .set_window_hint = NULL,
    .get_bitmap = gdc_rgb565_get_bitmap,
#if defined(USE_DMA2D) && !defined(TREZOR_EMULATOR)
    .fill = dma2d_rgb565_fill,
    .copy_mono4 = dma2d_rgb565_copy_mono4,
    .copy_rgb565 = gdc_rgb565_copy_rgb565,  // dma2d_rgb565_copy_rgb565,
    .copy_rgba8888 = NULL,
    .blend_mono4_mono4 = gdc_rgb565_blend_mono4_mono4,
    .blend_mono4_rgb565 = dma2d_rgb565_blend_mono4_rgb565,
    .blend_mono4_rgba8888 = NULL,
#else
    .fill = gdc_rgb565_fill,
    .copy_mono4 = gdc_rgb565_copy_mono4,
    .copy_rgb565 = gdc_rgb565_copy_rgb565,
    .copy_rgba8888 = NULL,
    .blend_mono4_mono4 = gdc_rgb565_blend_mono4_mono4,
    .blend_mono4_rgb565 = gdc_rgb565_blend_mono4_rgb565,
    .blend_mono4_rgba8888 = NULL,
#endif
  };

  gdc_bitmap_t bitmap = {.vmt = &gdc_rgb565,
                         .ptr = data_ptr,
                         .stride = stride,
                         .size = size,
                         .format = GDC_FORMAT_RGB565,
                         .attrs = attrs};

  return bitmap;
}
