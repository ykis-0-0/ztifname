use clap::Parser;
use eyre::{eyre, Result as eResult, Context as _};

use ::ztifname as lib;
use lib::RawNwid;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  /// Output FreeBSD interface name
  #[arg(long)]
  pub freebsd: bool,

  /// Output Linux interface name
  #[arg(long)]
  pub linux: bool,

  /// Extrapolate against collision possibilities,
  /// default to 1 if COUNT not provided
  #[arg(short, long, value_name = "COUNT", verbatim_doc_comment)]
  #[arg(requires = "linux")]
  pub extra: Option<Option<u8>>,

  /// ZeroTier Network ID, a 16-character long hexadecimal ASCII string
  nwid: String,
}

impl Cli {
  pub fn get_nwid(&self) -> eResult<RawNwid> {
    let nwid = self.nwid.as_str();

    if !nwid.is_ascii() {
      return Err(eyre!(
        "Network ID not a 16-character long hexadecimal ASCII string"
      ));
    }

    if nwid.len() != 16 {
      return Err(eyre!(
        "Network ID not hexadecimal 16-character long string"
      ));
    }

    RawNwid::from_str_radix(nwid, 16).wrap_err(
      "Cannot parse hex digits from provided ID"
    )
  }
}
