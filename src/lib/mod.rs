pub type RawNwid = u64;

pub mod freebsd;
pub mod linux;

#[cfg(target_os = "freebsd")]
pub use self::freebsd::ztdevname;

#[cfg(target_os = "linux")]
pub use self::linux::ztdevname;
