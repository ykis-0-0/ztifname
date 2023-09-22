use eyre::Result as eResult;

use ::ztifname as lib;
use lib::RawNwid;

mod cli;
use cli::Cli;

pub fn main() -> eResult<()> {
  let args = <Cli as clap::Parser>::parse();

  color_eyre::install()?;

  let nwid_raw: RawNwid = args.get_nwid()?;

  // Unspecified, print default ifname on OS and quits
  #[cfg(any(target_os = "linux", target_os = "freebsd"))]
  if !args.linux && !args.freebsd {
    println!("{}", lib::ztdevname(nwid_raw));
    return Ok(());
  }

  #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
  if !args.linux && !args.freebsd {
    use clap::error::ErrorKind::MissingRequiredArgument;

    <Cli as clap::CommandFactory>::command()
    .error(
      MissingRequiredArgument,
      "Specify target OS for ifname evaluation"
    )
    .exit();
  }

  if args.freebsd {
    println!("FreeBSD\t{}", lib::freebsd::ztdevname(nwid_raw));
  }

  if args.linux {
    let end_idx = match args.extra {
      None => 0,
      Some(None) => 1,
      Some(Some(count)) => count,
    };

    for pad in 0 ..= end_idx {
      println!("Linux\t{}", lib::linux::ztdevname(nwid_raw + pad as u64));
    }
  }

  Ok(())
}
