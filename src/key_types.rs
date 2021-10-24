// Copyright (c) 2021 The CobbleDB Authors. All rights reserved.

/* Key operation is very important to a key-value store.
 * Dedup technique such as prefix lifting are widely-used.
 * It makes sense to have a separate file keys.
 */
use std::cmp::Ordering;
use crate::defines::MSN;
use crate::cmp::Cmp;
//use integer_encoding::{FixedInt, VarInt};
use integer_encoding::{FixedInt, FixedIntWriter, VarInt, VarIntWriter};
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum MsgType {
    CbMsgDel = 0,
    CbMsgInsert = 1,
}

/// An Memtablekey has three major components:
/// [keylen, key, TAG, (vallen, val)] and TAG
/// is 8 bytes. (vallen, val) is option if the
/// mutation is a deletion.
pub type MemtableKey<'a> = &'a [u8];

/// The passed down by the user of the kv APIs.
pub type ClientKey<'a> = &'a [u8];

/// InternalKey contains [key, TAG].
/// It is used by the iterator of Memtable
pub type InternalKey<'a> = &'a [u8];

/// A QueryKey contains [keylen, key, tag]
/// keylen = len(key) + len(tag).
/// This is for the compatibility with LevelDB. 
#[derive(Clone, Debug)]
pub struct QueryKey {
    key: Vec<u8>,
    key_offset: usize,
}

const U64_SIZE: usize = 8;

impl QueryKey {
    pub fn new(k: ClientKey, msn: MSN) -> QueryKey {
        QueryKey::new_raw(k, msn, MsgType::CbMsgInsert)   
    }

    // TODO: need to evaluate the performance of these memcpy
    pub fn new_raw(k: ClientKey, msn: MSN, t: MsgType) -> QueryKey{
        let internal_keylen = k.len() + U64_SIZE;
        let mut key = Vec::new();
        key.resize(k.len() + internal_keylen.required_space() + U64_SIZE, 0);
        
        {
            let mut writer = key.as_mut_slice();
            writer.write_varint(internal_keylen)
                  .expect("write to slice failed");

            writer.write_all(k).expect("write to slice failed");
            writer.write_fixedint(msn << 8 | t as u64)
                  .expect("write to slice failed");
        }

        QueryKey {
            key,
            key_offset: internal_keylen.required_space(),
        }
    }

    pub fn memtable_key(&self) -> MemtableKey {
        self.key.as_slice()
    }

    pub fn client_key(&self) -> ClientKey {
        &self.key[self.key_offset..self.key.len() - 8]
    }

    pub fn internal_key(&self) -> InternalKey {
        &self.key[self.key_offset..]
    }
}

/// Parse a tag into (type, MSN)
pub fn parse_tag(tag: u64) -> (MsgType, u64) {
    let msn = tag >> 8;
    let mt =  tag & 0xff;

    match mt {
        0 => (MsgType::CbMsgDel, msn),
        1 => (MsgType::CbMsgInsert, msn),
        _ => (MsgType::CbMsgInsert, msn),
    }
}

pub fn build_memtable_key(key: &[u8], mt: MsgType, msn: MSN) -> Vec<u8> {
    /* Using the original LevelDB format:
     * [key_size: varint32, key_data: [u8], flags: u64, value_size: varint32, value_data: [u8]]
     */
    let keysize = key.len() + U64_SIZE; 
    let mut buf = Vec::new();

    buf.resize(
        keysize + keysize.required_space(),
        0,
    );

    {
        let mut writer = buf.as_mut_slice();
        writer.write_varint(keysize).expect("write to slice failed");
        writer.write_all(key).expect("write to slice failed");
        writer.write_fixedint((mt as u64) | (msn << 8))
              .expect("write to slice failed");
        assert_eq!(writer.len(), 0);
    }
    buf
}

/// comparion function for MemtableKey
pub fn cmp_memtable_key<'a, 'b>(
    ucmp: &dyn Cmp,
    a: MemtableKey<'a>,
    b: MemtableKey<'b>,
) -> Ordering {
    let (al, ao): (usize, usize) = VarInt::decode_var(&a);
    let (bl, bo): (usize, usize) = VarInt::decode_var(&b);
    let clientkey_a = &a[ao..ao + al - 8];
    let clientkey_b = &b[bo..bo + bl - 8];

    match ucmp.cmp(clientkey_a, clientkey_b) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            let atag = FixedInt::decode_fixed(&a[ao + al - 8..ao + al]);
            let btag = FixedInt::decode_fixed(&b[ao + bl - 8..bo + bl]);
            let (_, aseq) = parse_tag(atag);
            let (_, bseq) = parse_tag(btag);

            bseq.cmp(&aseq)
        }
    }
}
