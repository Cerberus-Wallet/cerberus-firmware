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

#ifndef GDC_DMA2D_H
#define GDC_DMA2D_H

#include <stdbool.h>
#include <stdint.h>

#include "gdc_color.h"

typedef struct {
  // Destination rectangle
  // (this fill are used always)
  uint16_t height;
  uint16_t width;
  void* dst_row;
  uint16_t dst_x;
  uint16_t dst_y;
  uint16_t dst_stride;

  // Source A (foreground) - only for copying
  // (srca_fg color for used for filling)
  void* srca_row;
  uint16_t srca_x;
  uint16_t srca_stride;
  gdc_color_t srca_fg;
  gdc_color_t srca_bg;

  // Source B (background) - only for blending
  void* srcb_row;
  uint16_t srcb_x;
  uint16_t srcb_stride;
  gdc_color_t srcb_fg;
  gdc_color_t srcb_bg;

  // Manipulate data by CPU only (memory is not accessible by DMA)
  bool cpu_only;
} dma2d_params_t;

#endif  // GDC_DMA2D_H