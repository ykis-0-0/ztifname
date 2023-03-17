use bitvec::{field::BitField, mem::bits_of, prelude::Msb0, view::BitView};

use super::Nwid;

// scraped from https://github.com/zerotier/ZeroTierOne/blob/4a4c8f84d50273f80050b665d99804662c8aa6bb/osdep/LinuxEthernetTap.cpp#L99
const BASE32_CHARS: &str = "abcdefghijklmnopqrstuvwxyz234567";

pub fn ztdevname(nwid: Nwid) -> String {
  // concentrate the effect to the smallest 40 bits
  let nwid_crossed = nwid ^ nwid >> 24;

  // and distill it out
  let bits = &nwid_crossed.view_bits::<Msb0>()[bits_of::<Nwid>() - 40 ..];

  let output = bits.chunks(5).map(|chunk| -> char {
    let idx = chunk.load_be::<usize>();
    BASE32_CHARS.as_bytes()[idx] as char
  });

  "zt".chars()
  .chain(output)
  .collect::<String>()
}

#[test]
fn validate() {
  assert_eq!(ztdevname(0xabfd_31bd_47a5_6b31), concat!("zt", "c25jjvtw"));
}
