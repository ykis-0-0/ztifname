use eyre::Result as eResult;

use ::ztifname as lib;
use lib::Nwid;

mod cli;
use cli::Cli;

pub fn main() -> eResult<()> {
  let args = <Cli as clap::Parser>::parse();

  color_eyre::install()?;

  let nwid: Nwid = args.get_nwid()?;

  // Unspecified, print default ifname on OS and quits
  if !args.linux && !args.freebsd {
    println!("{}", lib::ztdevname(nwid));
    return Ok(());
  }

  if args.freebsd {
    println!("FreeBSD\t{}", lib::freebsd::ztdevname(nwid));
  }

  if args.linux {
    let end_idx = match args.extra {
      None => 0,
      Some(None) => 1,
      Some(Some(count)) => count,
    };

    for pad in 0 ..= end_idx {
      println!("Linux\t{}", lib::linux::ztdevname(nwid + pad as u64));
    }
  }

  Ok(())
}
