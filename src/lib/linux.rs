use bitvec::{
  prelude::{BitVec, Msb0},
  field::BitField as _
};

use super::{Nwid, RawNwid, CellElem};

// scraped from https://github.com/zerotier/ZeroTierOne/blob/4a4c8f84d50273f80050b665d99804662c8aa6bb/osdep/LinuxEthernetTap.cpp#L99
const BASE32_CHARS: &str = "abcdefghijklmnopqrstuvwxyz234567";

pub fn ztdevname(nwid: Nwid) -> String {

  let nwid_crossed: BitVec<_, Msb0> = {
    type BitVecLE = BitVec<CellElem, Msb0>;
    // concentrate the effect to the smallest 40 bits
    const EFFECT_WIDTH: usize = 40;

    let idvec: BitVecLE = BitVec::from(nwid);
    let head = &idvec[ .. EFFECT_WIDTH];
    let tail = &idvec[RawNwid::BITS as usize - EFFECT_WIDTH .. ];

    // and distill it out
    head.to_owned() ^ tail
  };

  let output = nwid_crossed.chunks(5).map(|chunk| -> char {
    let idx = chunk.load_be::<usize>();
    BASE32_CHARS.as_bytes()[idx] as char
  });

  "zt".chars()
  .chain(output)
  .collect::<String>()
}

#[test]
fn sanity_check() {
  assert_eq!(ztdevname(Nwid::from(0x0123_4567_89ab_cdef)), concat!("zt", "m2vo5ktg"));
}

#[test]
fn validate() {
  assert_eq!(ztdevname(Nwid::from(0xabfd_31bd_47a5_6b31)), concat!("zt", "c25jjvtw"));
}
