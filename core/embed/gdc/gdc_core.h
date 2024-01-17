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

#ifndef GDC_CORE_H
#define GDC_CORE_H

#include "gdc_bitmap.h"
#include "gdc_color.h"
#include "gdc_dma2d.h"
#include "gdc_geom.h"

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// ------------------------------------------------------------------------
// GDC - Graphics Device Context
//

typedef void gdc_t;

// ------------------------------------------------------------------------
// GDC (Graphic Device Context) Virtual Method Table
//
// GDC structure is implementation specific. Only requirement is that
// it starts with a field of type gdc_vmt_t* vmt.
//
//    typedef struct
//    {
//        gdc_vmt_t* vmt;
//
//        // GDC specific data
//
//    } gdc_impl_specific_t;
//

typedef void (*gdc_release_t)(gdc_t* gdc);
typedef gdc_bitmap_t* (*gdc_get_bitmap_t)(gdc_t* gdc);
typedef void (*gdc_set_window_hint_t)(gdc_t* gdc, dma2d_params_t* params);
typedef bool (*gdc_fill_t)(gdc_t* gdc, dma2d_params_t* params);
typedef bool (*gdc_copy_mono4_t)(gdc_t* gdc, dma2d_params_t* params);
typedef bool (*gdc_copy_rgb565_t)(gdc_t* gdc, dma2d_params_t* params);
typedef bool (*gdc_copy_rgba8888_t)(gdc_t* gdc, dma2d_params_t* params);
typedef bool (*gdc_blend_mono4_mono4_t)(gdc_t* gdc, dma2d_params_t* params);
typedef bool (*gdc_blend_mono4_rgb565_t)(gdc_t* gdc, dma2d_params_t* params);
typedef bool (*gdc_blend_mono4_rgba8888_t)(gdc_t* gdc, dma2d_params_t* params);

// GDC virtual methods
struct gdc_vmt {
  gdc_release_t release;
  gdc_get_bitmap_t get_bitmap;
  gdc_set_window_hint_t set_window_hint;
  gdc_fill_t fill;
  gdc_copy_mono4_t copy_mono4;
  gdc_copy_rgb565_t copy_rgb565;
  gdc_copy_rgba8888_t copy_rgba8888;
  gdc_blend_mono4_mono4_t blend_mono4_mono4;
  gdc_blend_mono4_rgb565_t blend_mono4_rgb565;
  gdc_blend_mono4_rgba8888_t blend_mono4_rgba8888;
};

// ------------------------------------------------------------------------
// GDC (Graphic Device Context) Public API

// Releases reference to GDC
void gdc_release(gdc_t* gdc);

// Gets size of GDC bounding rectangle
gdc_size_t gdc_get_size(const gdc_t* gdc);

// Returns the bitmap format associated with the specified GDC
gdc_format_t gdc_get_format(const gdc_t* gdc);

// Wait for pending DMA operation applied on this GDC
// (used by high level code before accessing GDC's framebuffer/bitmap)
void gdc_wait_for_pending_ops(gdc_t* gdc);

// Sets the hint for the driver regarding the target windows beeing filled
// Some drivers may utilize this hint to optimize access to the
// display framebuffer
void gdc_set_window_hint(gdc_t* gdc, gdc_rect_t rect);

// Fills a rectangle with a specified color
bool gdc_fill_rect(gdc_t* gdc, gdc_rect_t rect, gdc_color_t color);

// Draws a bitmap into the specified rectangle. The destination rectangle
// may not be fully filled if the source bitmap is smaller then destination
// rectangle or if the bitmap is translated by an offset partially or completely
// outside the destination rectangle.
bool gdc_draw_bitmap(gdc_t* gdc, gdc_rect_t rect, const gdc_bitmap_ref_t* src);

// Draw a combination of two bitmaps into the specified rectangle
// Copies blended pixels of foreground(fg) over the background(bg)
// to the GDC window. If fg and bg bitmaps have different dimension or
// are translated by different offset, only the intersection of them is drawn.
bool gdc_draw_blended(gdc_t* gdc, gdc_rect_t rect, const gdc_bitmap_ref_t* fg,
                      const gdc_bitmap_ref_t* bg);

// ------------------------------------------------------------------------
// this will be defined elsewhere::

// Gets GDC for the hardware display
// Returns NULL if display gdc was already acquired and not released
gdc_t* display_acquire_gdc(void);

#endif  // GDC_CORE_H
