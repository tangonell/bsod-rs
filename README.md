# bsod-rs
[![License: 0BSD](https://img.shields.io/badge/License-0BSD-blue.svg)](https://opensource.org/licenses/0BSD)

Blazingly fast Blue<sup>1</sup><sup>2</sup> Screen of Death, written in 100% unsafe Rust 🔥

## Disclaimer
Running this *will* crash your Windows machine.
This project is intended for educational purposes only.

## Features
- **Blazingly fast:** Rust go brrr
- **Memory unsafety:** 0% safe code
- **No dependencies:** No crates, no std
- **Tiny size:** 10.0KiB (release build)

## How it works
This project uses undocumented `ntdll` functions (`RtlAdjustPrivilege` and `NtRaiseHardError`)
via FFI to raise a BSoD<sup>2</sup>.

## Size optimization
See `Cargo.toml`.

## Building from source
```shell
cargo build --release
```

## Troubleshooting
**Q:** Why has my machine crashed?\
**A:** It's working as intended.

**Q:** Is Linux support planned?\
**A:** No.

**Q:** Is this malware?\
**A:** No.

**Q:** Why not write this in C?\
**A:** Because I love Rust.

---

<sup>1</sup> Black Screen of Death on Windows 11\
<sup>2</sup> Green Screen of Death on Insider Preview builds