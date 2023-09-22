use bitvec::{
  prelude::{BitSlice, BitVec, BitOrder},
  field::BitField,
  mem::elts
};

pub type RawNwid = u64;
pub(super) type CellElem = usize;

#[derive(Debug, Clone)]
pub struct Nwid(RawNwid);

impl Nwid {
  pub fn size(&self) -> usize {
    RawNwid::BITS as usize
  }
}

impl From<RawNwid> for Nwid {
  fn from(source: RawNwid) -> Self {
    Self(source)
  }
}

impl<O> From<Nwid> for BitVec<CellElem, O> where
  O: BitOrder,
  // `S: BitStore` already satisfies the requirement,
  // but rustc didn't figure it out, so here we are
  BitSlice<CellElem, O>: BitField
{
    fn from(value: Nwid) -> Self {

      const CELLS: usize = elts::<CellElem>(RawNwid::BITS as usize);

      let mut rtv = BitVec::<CellElem, O>::from_vec(
        Vec::<CellElem>::from(
          [Default::default(); CELLS]
        )
      );

      rtv.store_be(value.0);

      rtv
    }
}
