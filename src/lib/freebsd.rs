use bitvec::{
  prelude::{BitVec, BitSlice, Msb0},
  field::BitField as _,
};

use super::{Nwid, CellElem};

// scraped from https://github.com/zerotier/ZeroTierOne/blob/4a4c8f84d50273f80050b665d99804662c8aa6bb/osdep/BSDEthernetTap.cpp#L55
const BASE32_CHARS: &str = "0123456789abcdefghijklmnopqrstuv";

pub fn ztdevname(nwid: Nwid) -> String {
  type BOrder = Msb0;

  let mapper = |chunk: &BitSlice<CellElem, BOrder>| -> char {
    let idx = chunk.load_be::<usize>();
    BASE32_CHARS.as_bytes()[idx] as char
  };

  let idvec = {
    let mut train = BitVec::<CellElem, BOrder>::from(nwid);

    // Unshift 1 bit to make even for 5bits x 13
    train.insert(0, false);

    train
  };

  // first char has to have 1 zero bit unshifted to make room for base32
  const CHAR_WIDTH: usize = 5;
  let chunks = idvec.chunks(CHAR_WIDTH).map(mapper);

  "zt".chars()
  .chain(chunks)
  .collect::<String>()
}

#[test]
fn sanity_check() {
  assert_eq!(ztdevname(Nwid::from(0x0123_4567_89ab_cdef)), concat!("zt", "028q5cu4qnjff"));
}

#[test]
fn validate() {
  assert_eq!(ztdevname(Nwid::from(0xabfd_31bd_47a5_6b31)), concat!("zt", "anv9hnl3qaqph"));
}
