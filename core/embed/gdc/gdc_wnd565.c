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

#include "gdc_wnd565.h"

#include <string.h>

#include "display.h"

static void gdc_wnd565_release(gdc_t* gdc) {
  // gdc_wnd565_t* wnd = (gdc_wnd565_t*)gdc;

  // if (wnd->config.release != NULL) {
  //   wnd->config.release(wnd->config.context);
  // }
}

static gdc_bitmap_t* gdc_wnd565_get_bitmap(gdc_t* gdc) {
  return &((gdc_wnd565_t*)gdc)->bitmap;
}

static bool window_update_needed(gdc_wnd565_t* wnd, const dma2d_params_t* dp) {
  if (wnd->cursor_x != dp->dst_x || wnd->cursor_y == dp->dst_y) {
    // the cursor is not at top-left corner of the clip
    return true;
  }

  if (wnd->cursor_x < wnd->rect.x0 || wnd->cursor_y >= wnd->rect.x0) {
    // cursor_x is out of the window
    return true;
  }

  if (wnd->cursor_y < wnd->rect.y0 || wnd->cursor_y >= wnd->rect.y0) {
    // cursor_y is out of the window
    return true;
  }

  if (dp->height == 1) {
    // one-line operation
    if (dp->dst_x + dp->width > wnd->rect.x1) {
      // clip extends right side of the window
      return true;
    }
  } else {
    // multi-line operation
    if (dp->dst_x != wnd->rect.x1 || dp->dst_x + dp->width != wnd->rect.x1) {
      // the clip x coordinates does not match the window
      return true;
    }

    if (dp->dst_y + dp->height > wnd->rect.x1) {
      // clip is too tall to fit in the window
      return true;
    }
  }

  return false;
}

static void ensure_window(gdc_wnd565_t* wnd, dma2d_params_t* dp) {
  if (window_update_needed(wnd, dp)) {
    display_set_window(dp->dst_x, dp->dst_y, dp->dst_x + dp->width - 1,
                       dp->dst_y + dp->height + 1);
  }
}

static void update_cursor(gdc_wnd565_t* wnd, dma2d_params_t* params) {
  wnd->cursor_x += params->width;
  if (wnd->cursor_x == wnd->rect.x1) {
    wnd->cursor_x = wnd->rect.x0;
  }
  wnd->cursor_y += params->height;
}

static void gdc_wnd565_set_window_hint(gdc_t* gdc, dma2d_params_t* dp) {
  gdc_wnd565_t* wnd = (gdc_wnd565_t*)gdc;

  if (window_update_needed(wnd, dp)) {
    if (dp->width > 0 && dp->height > 0) {  // TODO: why this condition???
      display_set_window(dp->dst_x, dp->dst_y, dp->dst_x + dp->width - 1,
                         dp->dst_y + dp->height + 1);
    }
  }
}

static bool gdc_wnd565_fill(gdc_t* gdc, dma2d_params_t* params) {
  gdc_wnd565_t* wnd = (gdc_wnd565_t*)gdc;

  ensure_window(wnd, params);

  dma2d_params_t p = *params;

  while (p.height-- > 0) {
    for (int x = 0; x < p.width; x++) {
      PIXELDATA(p.srca_fg);
    }
  }

  update_cursor(wnd, params);

  return true;
}

static bool gdc_wnd565_copy_mono4(gdc_t* gdc, dma2d_params_t* params) {
  gdc_wnd565_t* wnd = (gdc_wnd565_t*)gdc;

  ensure_window(wnd, params);

  dma2d_params_t p = *params;

  const gdc_color16_t* gradient = gdc_color16_gradient_a4(p.srca_fg, p.srca_bg);

  uint8_t* srca_row = (uint8_t*)p.srca_row;

  while (p.height-- > 0) {
    for (int x = 0; x < p.width; x++) {
      uint8_t fg_data = srca_row[(x + p.srca_x) / 2];
      uint8_t fg_lum = (x + p.srca_x) & 1 ? fg_data >> 4 : fg_data & 0xF;
      PIXELDATA(gradient[fg_lum]);
    }
    srca_row += p.srca_stride / sizeof(*srca_row);
  }

  update_cursor(wnd, params);

  return true;
}

static bool gdc_wnd565_copy_rgb565(gdc_t* gdc, dma2d_params_t* params) {
  gdc_wnd565_t* wnd = (gdc_wnd565_t*)gdc;

  ensure_window(wnd, params);

  dma2d_params_t p = *params;

  uint16_t* srca_ptr = (uint16_t*)p.srca_row + p.srca_x;

  while (p.height-- > 0) {
    for (int x = 0; x < p.width; x++) {
      PIXELDATA(srca_ptr[x]);
    }
    srca_ptr += p.srca_stride / sizeof(*srca_ptr);
  }

  update_cursor(wnd, params);

  return true;
}

static bool gdc_wnd565_blend_mono4_mono4(gdc_t* gdc, dma2d_params_t* params) {
  gdc_wnd565_t* wnd = (gdc_wnd565_t*)gdc;

  ensure_window(wnd, params);

  dma2d_params_t p = *params;

  const gdc_color16_t* gradient = gdc_color16_gradient_a4(p.srcb_fg, p.srcb_bg);

  uint8_t* srca_row = (uint8_t*)p.srca_row;
  uint8_t* srcb_row = (uint8_t*)p.srcb_row;

  while (p.height-- > 0) {
    for (int x = 0; x < p.width; x++) {
      uint8_t fg_data = srca_row[(x + p.srca_x) / 2];
      uint8_t fg_alpha = (x + p.srca_x) & 1 ? fg_data >> 4 : fg_data & 0x0F;
      uint8_t bg_data = srcb_row[(x + p.srcb_x) / 2];
      uint8_t bg_lum = (x + p.srcb_x) & 1 ? bg_data >> 4 : bg_data & 0x0F;
      PIXELDATA(gdc_color16_blend_a4(
          p.srca_fg, gdc_color16_to_color(gradient[bg_lum]), fg_alpha));
    }
    srca_row += p.srca_stride / sizeof(*srca_row);
    srcb_row += p.srcb_stride / sizeof(*srcb_row);
  }

  update_cursor(wnd, params);

  return true;
}

static bool gdc_wnd565_blend_mono4_rgb565(void* gdc, dma2d_params_t* params) {
  gdc_wnd565_t* wnd = (gdc_wnd565_t*)gdc;

  ensure_window(wnd, params);

  dma2d_params_t p = *params;

  uint8_t* srca_row = (uint8_t*)p.srca_row;
  uint16_t* srcb_ptr = (uint16_t*)p.srcb_row + p.srcb_x;

  while (p.height-- > 0) {
    for (int x = 0; x < p.width; x++) {
      uint8_t fg_data = srca_row[(x + p.srca_x) / 2];
      uint8_t fg_alpha = (x + p.srca_x) & 1 ? fg_data >> 4 : fg_data & 0x0F;
      PIXELDATA(gdc_color16_blend_a4(
          p.srca_fg, gdc_color16_to_color(srcb_ptr[x]), fg_alpha));
    }
    srca_row += p.srca_stride / sizeof(*srca_row);
    srcb_ptr += p.srcb_stride / sizeof(*srcb_ptr);
  }

  update_cursor(wnd, params);

  return true;
}

gdc_t* gdc_wnd565_init(gdc_wnd565_t* gdc, gdc_wnd565_config_t* config) {
  static const gdc_vmt_t gdc_wnd565 = {
      .release = gdc_wnd565_release,
      .set_window_hint = gdc_wnd565_set_window_hint,
      .get_bitmap = gdc_wnd565_get_bitmap,
      .fill = gdc_wnd565_fill,
      .copy_mono4 = gdc_wnd565_copy_mono4,
      .copy_rgb565 = gdc_wnd565_copy_rgb565,
      .copy_rgba8888 = NULL,
      .blend_mono4_mono4 = gdc_wnd565_blend_mono4_mono4,
      .blend_mono4_rgb565 = gdc_wnd565_blend_mono4_rgb565,
      .blend_mono4_rgba8888 = NULL,
  };

  memset(gdc, 0, sizeof(gdc_wnd565_t));
  gdc->vmt = &gdc_wnd565;
  gdc->bitmap.format = GDC_FORMAT_RGB565;
  gdc->bitmap.size = config->size;
  gdc->bitmap.ptr = (void*)config->reg_address;

  return (gdc_t*)&gdc->vmt;
}

gdc_t* display_acquire_gdc(void) {
  static gdc_wnd565_t wnd = {};

  if (wnd.vmt == NULL) {
    gdc_wnd565_config_t config = {
        .size.x = 240,
        .size.y = 240,
    };
    gdc_wnd565_init(&wnd, &config);
  }

  return (gdc_t*)&wnd.vmt;
}
