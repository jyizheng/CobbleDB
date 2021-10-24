// Copyright (c) 2021 The CobbleDB Authors. All rights reserved.

use std::cmp::Ordering;

/// The trait for comparator 
pub trait Cmp {
    /// Compare byte array
    fn cmp(&self, a: &[u8], b: &[u8]) -> Ordering;

    /// A Id for a comparator
    fn id(&self) -> &'static str;
}

/// The default byte-wise comparator
#[derive(Clone)]
pub struct DefaultCmp;

impl Cmp for DefaultCmp {
    fn cmp(&self, a: &[u8], b: &[u8]) -> Ordering {
        a.cmp(b)
    }

    fn id(&self) -> &'static str {
        "CobbleDB.BytewiseComparator"
    }
}

