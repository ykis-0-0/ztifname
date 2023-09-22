use bitvec::{field::BitField, prelude::Msb0, slice::BitSlice, view::BitView};

use super::Nwid;

// scraped from https://github.com/zerotier/ZeroTierOne/blob/4a4c8f84d50273f80050b665d99804662c8aa6bb/osdep/BSDEthernetTap.cpp#L55
const BASE32_CHARS: &str = "0123456789abcdefghijklmnopqrstuv";

pub fn ztdevname(nwid: Nwid) -> String {

  let bits = nwid.view_bits::<Msb0>();

  let expanded = {
    let mut rtv = bits.to_bitvec();
    rtv.insert(0, false);
    rtv
  };

  let mapper = |chunk: &BitSlice<Nwid, Msb0>| -> char {
    let idx = chunk.load_be::<usize>();
    let rtv = BASE32_CHARS.as_bytes()[idx] as char;
    rtv
  };

  // first char has to have 1 zero bit unshifted to make room for base32
  let char0 = std::iter::once(mapper(&(bits[.. 4])));
  let remaining = expanded[5 ..].chunks(5).map(mapper);

  "zt"
    .chars()
    .chain(char0)
    .chain(remaining)
    .collect::<String>()
}

#[test]
fn sanity_check() {
  assert_eq!(ztdevname(0x0123_4567_89ab_cdef), concat!("zt", "028q5cu4qnjff"));
}

#[test]
fn validate() {
  assert_eq!(ztdevname(0xabfd_31bd_47a5_6b31), concat!("zt", "anv9hnl3qaqph"));
}
