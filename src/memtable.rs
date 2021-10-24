// Copyright (c) 2021 The CobbleDB Authors. All rights reserved.

use std::collections::btree_map::Iter;
use crate::key_types::build_memtable_key;
use crate::key_types::MsgType;
use crate::defines::MSN;
use crate::key_types::MemtableKey;
use std::collections::BTreeMap;
//use std::ops::Bound::Included;
use crate::key_types::{QueryKey, ClientKey};

/* There are several ordered in-memory data structure
 * can be considered to implement the memtable.
 * Btree is an easy one come into mind. Also, there are
 * other options implemented in RocksDB, such as skip list.
 * There are new ones in the literature:
 * (1) Wormhole (Eurosys'19)
 * (2) HydraList (VLDB'2020)
 *
 * BtreeMap is used as the baseline here.
 * FIXME: make these data structures as plug-ins
 */


/* Semantic of the Memtable:
 * (1) TLA+ specification
 * (2) MIRAI invariant.
 * TODO: Elaborate more on this
 */

pub struct MemTable<'a> {
    map: BTreeMap<MemtableKey<'a>, &'a[u8]>,
}

impl<'a> MemTable<'a> {
    pub fn new() -> MemTable<'static> {
        MemTable {
            map: BTreeMap::new(),
        }
    }  

    pub fn len(&self) -> usize {
        self.map.len()
    }   

    pub fn mem_foot_print(&self) -> usize {
        /*
         * BtreeMap does not have a straightforward API yet
         * TODO: This may be a good reason to extend BtreeMap
         */
        panic!("Not implemented yet");
    }

    pub fn put(&mut self, msn: MSN, mt: MsgType, key: ClientKey<'a>, value: &'a [u8]) {
        self.map.insert(&build_memtable_key(key, mt, msn), value);
    }

    pub fn get(&self, _key: &QueryKey) -> (Option<Vec<u8>>, bool) {
        //let mut iter = self.map.iter();
        //let range = self.map.range(key..);
        //let entry = range.next();
        //iter.seek(key.memtable_key());

        // FIXME: this is not correct
        //if let Some((foundk, foundv)) = entry {
        //    if key.client_key() == &foundv[..] {
        //        return (Some(foundk[..].to_vec()), false);
        //    } else {
        //        return (None, true);
        //    }
        //}
        (None, false)
    }

    pub fn iter(&self) -> Iter<MemtableKey, &'a[u8]> {
        self.map.iter()
    }
}
