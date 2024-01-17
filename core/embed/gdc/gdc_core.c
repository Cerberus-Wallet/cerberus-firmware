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
#include "gdc_clip.h"

#include "dma2d.h"

void gdc_release(gdc_t* gdc) {
  if (gdc != NULL) {
    gdc_wait_for_pending_ops(gdc);

    gdc_vmt_t* vmt = *(gdc_vmt_t**)gdc;
    if (vmt != NULL && vmt->release != NULL) {
      vmt->release(gdc);
    }
  }
}

gdc_size_t gdc_get_size(const gdc_t* gdc) {
  if (gdc != NULL) {
    gdc_vmt_t* vmt = *(gdc_vmt_t**)gdc;
    if (vmt != NULL && vmt->get_bitmap != NULL) {
      return vmt->get_bitmap((gdc_t*)gdc)->size;
    }
  }

  return gdc_size(0, 0);
}

gdc_format_t gdc_get_format(const gdc_t* gdc) {
  if (gdc != NULL) {
    gdc_vmt_t* vmt = *(gdc_vmt_t**)gdc;
    if (vmt != NULL && vmt->get_bitmap != NULL) {
      return vmt->get_bitmap((gdc_t*)gdc)->format;
    }
  }
  return GDC_FORMAT_UNKNOWN;
}

void gdc_wait_for_pending_ops(gdc_t* gdc) {
#if defined(USE_DMA2D) && !defined(TREZOR_EMULATOR)
  if (gdc != NULL) {
    dma2d_wait(gdc);
  }
#endif
}

void gdc_set_window_hint(gdc_t* gdc, gdc_rect_t rect) {
  if (gdc != NULL) {
    gdc_vmt_t* vmt = *(gdc_vmt_t**)gdc;

    gdc_bitmap_t* bitmap = (gdc_bitmap_t*)gdc;

    gdc_clip_t clip = gdc_clip(rect, bitmap->size, NULL, NULL);

    dma2d_params_t dp = {
        // Destination rectangle
        .height = clip.height,
        .width = clip.width,
        .dst_x = clip.dst_x,
        .dst_y = clip.dst_y,
    };

    // xgdc_wait_for_dma();

    return vmt->set_window_hint(gdc, &dp);
  }
}

bool gdc_fill_rect(gdc_t* gdc, gdc_rect_t rect, gdc_color_t color) {
  if (gdc != NULL) {
    gdc_vmt_t* vmt = *(gdc_vmt_t**)gdc;

    gdc_bitmap_t* bitmap = (gdc_bitmap_t*)gdc;

    gdc_clip_t clip = gdc_clip(rect, bitmap->size, NULL, NULL);

    if (clip.width <= 0 || clip.height <= 0) {
      return true;
    }

    dma2d_params_t dp = {
        // Destination rectangle
        .height = clip.height,
        .width = clip.width,
        .dst_row = (uint8_t*)bitmap->ptr + bitmap->stride * clip.dst_y,
        .dst_x = clip.dst_x,
        .dst_y = clip.dst_y,
        .dst_stride = bitmap->stride,

        // Source A (foreground)
        .srca_fg = color,

        .cpu_only = (bitmap->attrs & GDC_BITMAP_NO_DMA) != 0,
    };

    gdc_wait_for_pending_ops(gdc);

    if (vmt->fill != NULL) {
      return vmt->fill(gdc, &dp);
    }
  }

  return false;
}

bool gdc_draw_bitmap(gdc_t* gdc, gdc_rect_t rect, const gdc_bitmap_ref_t* fg) {
  if (gdc != NULL) {
    gdc_vmt_t* vmt = *(gdc_vmt_t**)gdc;

    gdc_bitmap_t* bitmap = vmt->get_bitmap(gdc);

    gdc_clip_t clip = gdc_clip(rect, bitmap->size, fg, NULL);

    if (clip.width <= 0 || clip.height <= 0) {
      return true;
    }

    dma2d_params_t dp = {
        // Destination rectangle
        .height = clip.height,
        .width = clip.width,
        .dst_row = (uint8_t*)bitmap->ptr + bitmap->stride * clip.dst_y,
        .dst_x = clip.dst_x,
        .dst_y = clip.dst_y,
        .dst_stride = bitmap->stride,

        // Source A (foreground)
        .srca_row = (uint8_t*)fg->bitmap->ptr + fg->bitmap->stride * clip.fg_y,
        .srca_x = clip.fg_x,
        .srca_stride = fg->bitmap->stride,
        .srca_fg = fg->fg_color,
        .srca_bg = fg->bg_color,

        .cpu_only = (bitmap->attrs & GDC_BITMAP_NO_DMA) ||
                    (fg->bitmap->attrs & GDC_BITMAP_NO_DMA),
    };

    gdc_wait_for_pending_ops(gdc);

    if (fg->bitmap->format == GDC_FORMAT_MONO4) {
      if (vmt->copy_mono4 != NULL) {
        return vmt->copy_mono4(gdc, &dp);
      }
    } else if (fg->bitmap->format == GDC_FORMAT_RGB565) {
      if (vmt->copy_rgb565 != NULL) {
        return vmt->copy_rgb565(gdc, &dp);
      }
    } else if (fg->bitmap->format == GDC_FORMAT_RGBA8888) {
      if (vmt->copy_rgba8888 != NULL) {
        return vmt->copy_rgba8888(gdc, &dp);
      }
    }
  }

  return false;
}

bool gdc_draw_blended(gdc_t* gdc, gdc_rect_t rect, const gdc_bitmap_ref_t* fg,
                      const gdc_bitmap_ref_t* bg) {
  if (gdc != NULL) {
    gdc_vmt_t* vmt = *(gdc_vmt_t**)gdc;

    gdc_bitmap_t* bitmap = vmt->get_bitmap(gdc);

    gdc_clip_t clip = gdc_clip(rect, bitmap->size, fg, bg);

    if (clip.width <= 0 || clip.height <= 0) {
      return true;
    }

    dma2d_params_t dp = {
        // Destination rectangle
        .height = clip.height,
        .width = clip.width,
        .dst_row = (uint8_t*)bitmap->ptr + bitmap->stride * clip.dst_y,
        .dst_x = clip.dst_x,
        .dst_y = clip.dst_y,
        .dst_stride = bitmap->stride,

        // Source A (foreground)
        .srca_row = (uint8_t*)fg->bitmap->ptr + fg->bitmap->stride * clip.fg_y,
        .srca_x = clip.fg_x,
        .srca_stride = fg->bitmap->stride,
        .srca_fg = fg->fg_color,

        // Source B (background)
        .srcb_row = (uint8_t*)bg->bitmap->ptr + bg->bitmap->stride * clip.bg_y,
        .srcb_x = clip.bg_x,
        .srcb_stride = bg->bitmap->stride,
        .srcb_fg = bg->fg_color,
        .srcb_bg = bg->bg_color,

        .cpu_only = (bitmap->attrs & GDC_BITMAP_NO_DMA) ||
                    (fg->bitmap->attrs & GDC_BITMAP_NO_DMA) ||
                    (fg->bitmap->attrs & GDC_BITMAP_NO_DMA),
    };

    gdc_wait_for_pending_ops(gdc);

    if (fg->bitmap->format == GDC_FORMAT_MONO4 &&
        bg->bitmap->format == GDC_FORMAT_RGB565) {
      if (vmt->blend_mono4_rgb565 != NULL) {
        return vmt->blend_mono4_rgb565(gdc, &dp);
      }
    } else if (fg->bitmap->format == GDC_FORMAT_MONO4 &&
               bg->bitmap->format == GDC_FORMAT_RGBA8888) {
      if (vmt->blend_mono4_rgba8888 != NULL) {
        return vmt->blend_mono4_rgba8888(gdc, &dp);
      }
    } else if (fg->bitmap->format == GDC_FORMAT_MONO4 &&
               bg->bitmap->format == GDC_FORMAT_MONO4) {
      if (vmt->blend_mono4_mono4 != NULL) {
        return vmt->blend_mono4_mono4(gdc, &dp);
      }
    }
  }

  return false;
}
