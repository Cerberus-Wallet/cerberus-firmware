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

#include "gdc_heap.h"

#include <stdbool.h>
#include <stdint.h>

typedef struct gdc_heap_free_item gdc_heap_free_item_t;

typedef struct heap_block {
  size_t size;
  struct heap_block* next;
} heap_block_t;

typedef struct {
  heap_block_t data[(24 * 1024) / sizeof(heap_block_t)];
  heap_block_t* free;
  bool initialized;
} heap_t;

heap_t gdc_heap;

void* gdc_heap_alloc(size_t size) {
  heap_t* heap = &gdc_heap;

  if (!heap->initialized) {
    heap->data[0].size = sizeof(heap->data);
    heap->data[0].next = NULL;
    heap->free = &heap->data[0];
    heap->initialized = true;
  }

  heap_block_t* block = heap->free;
  heap_block_t** prev = &heap->free;

  size = (size + 7) & ~0x7;

  while (block != NULL) {
    if (block->size == size) {
      // perfect fit
      *prev = block->next;
      return block;
    } else if (block->size > size) {
      // split block
      heap_block_t* new_block = (heap_block_t*)((uintptr_t)block + size);
      new_block->size = block->size - size;
      new_block->next = block->next;
      *prev = new_block;
      return block;
    }

    prev = &block->next;
    block = block->next;
  }

  return NULL;
}

void gdc_heap_free(void* ptr, size_t size) {
  heap_t* heap = &gdc_heap;

  size = (size + 7) & ~0x7;

  heap_block_t* next = heap->free;
  heap_block_t* prev = NULL;

  while (next != NULL && (uintptr_t)ptr >= (uintptr_t)next) {
    prev = next;
    next = next->next;
  }

  heap_block_t* block = (heap_block_t*)ptr;
  block->size = size;
  block->next = next;

  if (prev != NULL) {
    if ((uintptr_t)prev + prev->size == (uintptr_t)ptr) {
      // colaesce with the previous block
      prev->size += size;
      block = prev;
    } else {
      prev->next = block;
    }
  } else {
    heap->free = block;
  }

  if (block->next != NULL) {
    if ((uintptr_t)block + block->size == (uintptr_t)block->next) {
      // colaesce with the next block
      block->size += block->next->size;
      block->next = block->next->next;
    }
  }
}
