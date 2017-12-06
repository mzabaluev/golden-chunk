// Copyright 2017 Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::heap::Layout;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChunkLayout {
    layout: Layout,
    len: usize
}

impl From<ChunkLayout> for Layout {
    #[inline]
    fn from(cl: ChunkLayout) -> Layout {
        cl.layout
    }
}

impl<'a> From<&'a ChunkLayout> for Layout {
    #[inline]
    fn from(cl: &'a ChunkLayout) -> Layout {
        cl.layout.clone()
    }
}

impl AsRef<Layout> for ChunkLayout {
    #[inline]
    fn as_ref(&self) -> &Layout { &self.layout }
}

fn elem_size_align(layout: Layout) -> (usize, usize) {
    let align = layout.align();
    // Pad the element size to its own alignment.
    // This also guarantees a nonzero element size because any valid
    // alignment is a power of two. So we produce a workable, if
    // meaningless, layout for zero-sized elements.
    let size = layout.padding_needed_for(align);
    (size, align)
}

impl ChunkLayout {

    pub fn fit_capacity_or_one(
            elem_layout: Layout,
            capacity: usize)
            -> ChunkLayout
    {
        let (elem_size, elem_align) = elem_size_align(elem_layout);

        let mut len = capacity / elem_size;
        if len == 0 {
            len = 1;
        }

        // The multiplication never overflows because the result is either
        // below capacity or a valid layout size.
        // The total size is aligned to elements' alignment,
        // so this is always safe.
        let layout = unsafe {
            Layout::from_size_align_unchecked(
                elem_size * len, elem_align)
        };

        ChunkLayout { layout, len }
    }

    pub fn fit_capacity_or_one_po2(
            elem_layout: Layout,
            capacity: usize)
            -> (ChunkLayout, u32)
    {
        let (elem_size, elem_align) = elem_size_align(elem_layout);

        let cap_lz = capacity.leading_zeros();
        let elem_lz = elem_size.leading_zeros();
        let (total_size, po2) =
            if cap_lz < elem_lz {
                let mut shift = elem_lz - cap_lz;
                // Never overflows because it can only eat leading zeros
                let mut size = elem_size << shift;
                if size > capacity {
                    // Never overflows because shift is nonzero
                    shift -= 1;
                    // size can't be shifted below elem_size here
                    size >>= 1;
                }
                (size, shift)
            } else {
                // Can only fit one, or the element size is above capacity
                (elem_size, 0)
            };

        // The total size is aligned to elements' alignment,
        // so this is always safe.
        let layout = unsafe {
            Layout::from_size_align_unchecked(
                total_size, elem_align)
        };

        // The shift never overflows because
        // po2 is less than the number of bits in usize
        (ChunkLayout { layout, len: 1 << po2 }, po2)
    }

    #[inline]
    pub fn len(&self) -> usize { self.len }

    #[inline]
    pub fn elem(&self) -> Layout {
        // self.layout is always constructed as a repetition of the
        // element layout, so this is completely safe:
        unsafe {
            Layout::from_size_align_unchecked(
                self.layout.size() / self.len,
                self.layout.align())
        }
    }
}
