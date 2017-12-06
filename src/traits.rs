// Copyright 2017 Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use layout::ChunkLayout;
use std::heap::Layout;


pub trait FastChunk {
    fn fast_chunk_layout(&self, elem_layout: Layout) -> ChunkLayout;

    fn fast_array_layout<T>(&self) -> ChunkLayout {
        self.fast_chunk_layout(Layout::new::<T>())
    }
}

pub trait BulkChunk {
    fn bulk_chunk_layout(&self,
                         elem_layout: Layout,
                         overcommit_hint: usize)
                         -> ChunkLayout;

    fn bulk_array_layout<T>(&self,
                            overcommit_hint: usize)
                            -> ChunkLayout
    {
        self.bulk_chunk_layout(Layout::new::<T>(), overcommit_hint)
    }
}
