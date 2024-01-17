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

#ifndef GDC_CLIP_H
#define GDC_CLIP_H

#include <common.h>
#include "gdc.h"

typedef struct {
  int16_t dst_x;
  int16_t dst_y;
  int16_t fg_x;
  int16_t fg_y;
  int16_t bg_x;
  int16_t bg_y;
  int16_t width;
  int16_t height;
} gdc_clip_t;

static inline gdc_clip_t gdc_clip(gdc_rect_t dst, gdc_size_t size,
                                  const gdc_bitmap_ref_t* fg,
                                  const gdc_bitmap_ref_t* bg) {
  int16_t dst_x = dst.x0;
  int16_t dst_y = dst.y0;

  int16_t fg_x = 0;
  int16_t fg_y = 0;

  int16_t bg_x = 0;
  int16_t bg_y = 0;

  if (fg != NULL) {
    fg_x += fg->offset.x;
    fg_y += fg->offset.y;

    // Normalize negative x-offset of fg bitmap
    if (fg_x < 0) {
      dst_x -= fg_x;
      bg_x -= fg_x;
      fg_x = 0;
    }

    // Normalize negative y-offset of fg bitmap
    if (fg_y < 0) {
      dst_y -= fg_y;
      bg_y -= fg_y;
      fg_y = 0;
    }
  }

  if (bg != NULL) {
    bg_x += bg->offset.x;
    bg_y += bg->offset.y;

    // Normalize negative x-offset of bg bitmap
    if (bg_x < 0) {
      dst_x -= bg_x;
      fg_x -= bg_x;
      bg_x = 0;
    }

    // Normalize negative y-offset of bg bitmap
    if (bg_y < 0) {
      dst_y -= bg_y;
      fg_y -= bg_y;
      bg_y = 0;
    }
  }

  // Normalize negative top-left of destination rectangle
  if (dst_x < 0) {
    fg_x -= dst_x;
    bg_x -= dst_x;
    dst_x = 0;
  }

  if (dst_y < 0) {
    fg_y -= dst_y;
    bg_y -= dst_y;
    dst_y = 0;
  }

  // Calculate dimension of effective rectangle
  int16_t width = MIN(size.x, dst.x1) - dst_x;
  int16_t height = MIN(size.y, dst.y1) - dst_y;

  if (fg != NULL) {
    width = MIN(width, fg->bitmap->size.x - fg_x);
    height = MIN(height, fg->bitmap->size.y - fg_y);
  }

  if (bg != NULL) {
    width = MIN(width, bg->bitmap->size.x - bg_x);
    height = MIN(height, bg->bitmap->size.y - bg_y);
  }

  gdc_clip_t clip = {
      .dst_x = dst_x,
      .dst_y = dst_y,
      .fg_x = fg_x,
      .fg_y = fg_y,
      .bg_x = bg_x,
      .bg_y = bg_y,
      .width = width,
      .height = height,
  };

  return clip;
}

#endif  // GDC_CLIP_H
