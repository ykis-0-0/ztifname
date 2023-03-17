pub type Nwid = u64;

pub mod linux;
pub mod freebsd;

#[cfg(target_os = "linux")]
pub use self::linux::ztdevname;

#[cfg(target_os = "freebsd")]
pub use self::freebsd::ztdevname;
