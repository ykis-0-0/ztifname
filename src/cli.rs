use clap::Parser;
use eyre::{Result as eResult, eyre, Context};

use ::ztidtrans as lib;
use lib::Nwid;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  /// Output FreeBSD interface name
  #[arg(long)]
  pub freebsd: bool,

  /// Output Linux interface name
  #[arg(long)]
  pub linux: bool,

  /// Extrapolate against collision possibilities
  #[arg(short, long, value_name = "COUNT", default_value = "1")]
  #[arg(requires = "linux")]
  pub extra: Option<Option<u8>>,

  /*
  /// Verbosity of output, 0 for machine consumable minimal output
  #[arg(long = "verbose", action = clap::ArgAction::Count)]
  pub verbosity: u8,
  */

  /// ZeroTier Network ID, a 16-character long hexadecimal ASCII string
  nwid: String
}

impl Cli {
  pub fn get_nwid(&self) -> eResult<Nwid> {
    let nwid = self.nwid.as_str();

    if !nwid.is_ascii() {
      return Err(eyre!("Network ID not a 16-character long hexadecimal ASCII string"));
    }

    if nwid.len() != 16 {
      return Err(eyre!("Network ID not hexadecimal 16-character long string"));
    }

    match Nwid::from_str_radix(nwid, 16) {
        Ok(netid) => Ok(netid), // std::Result::Ok => eyre::Result::Ok
        err @ Err(_) => err.wrap_err("Cannot parse hex digits from provided ID"),
    }
  }
}
