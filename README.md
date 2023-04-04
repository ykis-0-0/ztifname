# [ZeroTier][zt] Interface Name Estimator

## **tl;dr:**
```console
[nixos@wslnix:~] $ ztifname --linux --freebsd 8056c2e21c123456
FreeBSD zt80lm2s8e14d2m
Linux   ztmjfnbvsk
```

```console
[nixos@wslnix:~] $ ztifname --help
Usage: ztifname [OPTIONS] <NWID>

Arguments:
  <NWID>  ZeroTier Network ID, a 16-character long hexadecimal ASCII string

Options:
      --freebsd          Output FreeBSD interface name
      --linux            Output Linux interface name
  -e, --extra [<COUNT>]  Extrapolate against collision possibilities,
                         default to 1 if COUNT not provided
  -h, --help             Print help
  -V, --version          Print version
```

## Why?
* ZeroTier networks have a [routing independent ID][zt.nwid],
* Every ZT network joined will create an interface with a name computed from the Network ID
* **But**, ZeroTier (the CLI) doesn't currently provide a way to inquire the interface name when we need it *prior to* joining a network
* Therefore I googled the source, copied the calculations, and made this 🤷🏿‍♂️

## How?
The [relevant sources](./src/lib/) should be short enough to read (a file for each OS, 70 lines in total) if interested. If you can't comprehend, basically its `base32` with some bit-jiggling black magic.

[zt]: https://www.zerotier.com/
[nixos]: https://nixos.org/
[zt.nwid]: https://docs.zerotier.com/zerotier/manual#221networkidentifiersandcontrollersaname2_2_1a
