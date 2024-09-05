# rusttest-to-dg

Converts `rustc` testcases into dejagnu testcases for `gccrs`

## DejaGnu equivalvent directives for Rustc test headers:

## [Controlling pass/fail expectations](ui.md#controlling-passfail-expectations)

| Rustc Directive   | DejaGnu Directive |
| ----------------- | ----------------- |
| check-pass        | DejaGnu directive |
| build-pass        | DejaGnu directive |
| run-pass          | DejaGnu directive |
| check-fail        | DejaGnu directive |
| build-fail        | DejaGnu directive |
| run-fail          | DejaGnu directive |
| ignore-pass       | DejaGnu directive |
| check-run-results | DejaGnu directive |

## [UI](ui.md) headers

| Rustc Directive                 | DejaGnu Directive |
| ------------------------------- | ----------------- |
| normalize-stderr-32bit          | DejaGnu directive |
| normalize-stderr-64bit          | DejaGnu directive |
| normalize-stderr-test           | DejaGnu directive |
| normalize-stdout-test           | DejaGnu directive |
| run-rustfix                     | DejaGnu directive |
| rustfix-only-machine-applicable | DejaGnu directive |
| stderr-per-bitwidth             | DejaGnu directive |
| dont-check-compiler-stderr      | DejaGnu directive |
| dont-check-compiler-stdout      | DejaGnu directive |
| compare-output-lines-by-subset  | DejaGnu directive |

## [Building auxiliary crates](compiletest.md#building-auxiliary-crates)

| Rustc Directive     | DejaGnu Directive |
| ------------------- | ----------------- |
| aux-build           | DejaGnu directive |
| aux-crate           | DejaGnu directive |
| aux-bin             | DejaGnu directive |
| aux-codegen-backend | DejaGnu directive |

## [Pretty-printer](compiletest.md#pretty-printer-tests) headers

| Rustc Directive     | DejaGnu Directive |
| ------------------- | ----------------- |
| pretty-compare-only | DejaGnu directive |
| pretty-expanded     | DejaGnu directive |
| pretty-mode         | DejaGnu directive |
| pp-exact            | DejaGnu directive |

## [Ignoring tests](#ignoring-tests)

| Rustc Directive                    | DejaGnu Directive |
| ---------------------------------- | ----------------- |
| ignore-16bit                       | DejaGnu directive |
| ignore-32bit                       | DejaGnu directive |
| ignore-64bit                       | DejaGnu directive |
| ignore-aarch64                     | DejaGnu directive |
| ignore-aarch64-unknown-linux-gnu   | DejaGnu directive |
| ignore-android                     | DejaGnu directive |
| ignore-apple                       | DejaGnu directive |
| ignore-arm                         | DejaGnu directive |
| ignore-avr                         | DejaGnu directive |
| ignore-beta                        | DejaGnu directive |
| ignore-cdb                         | DejaGnu directive |
| ignore-compare-mode-next-solver    | DejaGnu directive |
| ignore-compare-mode-polonius       | DejaGnu directive |
| ignore-cross-compile               | DejaGnu directive |
| ignore-debug                       | DejaGnu directive |
| ignore-eabi                        | DejaGnu directive |
| ignore-emscripten                  | DejaGnu directive |
| ignore-endian-big                  | DejaGnu directive |
| ignore-freebsd                     | DejaGnu directive |
| ignore-fuchsia                     | DejaGnu directive |
| ignore-gdb                         | DejaGnu directive |
| ignore-gdb-version                 | DejaGnu directive |
| ignore-gnu                         | DejaGnu directive |
| ignore-haiku                       | DejaGnu directive |
| ignore-horizon                     | DejaGnu directive |
| ignore-i686-pc-windows-gnu         | DejaGnu directive |
| ignore-i686-pc-windows-msvc        | DejaGnu directive |
| ignore-illumos                     | DejaGnu directive |
| ignore-ios                         | DejaGnu directive |
| ignore-linux                       | DejaGnu directive |
| ignore-lldb                        | DejaGnu directive |
| ignore-llvm-version                | DejaGnu directive |
| ignore-loongarch64                 | DejaGnu directive |
| ignore-macabi                      | DejaGnu directive |
| ignore-macos                       | DejaGnu directive |
| ignore-mode-assembly               | DejaGnu directive |
| ignore-mode-codegen                | DejaGnu directive |
| ignore-mode-codegen-units          | DejaGnu directive |
| ignore-mode-coverage-map           | DejaGnu directive |
| ignore-mode-coverage-run           | DejaGnu directive |
| ignore-mode-crashes                | DejaGnu directive |
| ignore-mode-debuginfo              | DejaGnu directive |
| ignore-mode-incremental            | DejaGnu directive |
| ignore-mode-js-doc-test            | DejaGnu directive |
| ignore-mode-mir-opt                | DejaGnu directive |
| ignore-mode-pretty                 | DejaGnu directive |
| ignore-mode-run-make               | DejaGnu directive |
| ignore-mode-run-pass-valgrind      | DejaGnu directive |
| ignore-mode-rustdoc                | DejaGnu directive |
| ignore-mode-rustdoc-json           | DejaGnu directive |
| ignore-mode-ui                     | DejaGnu directive |
| ignore-mode-ui-fulldeps            | DejaGnu directive |
| ignore-msp430                      | DejaGnu directive |
| ignore-msvc                        | DejaGnu directive |
| ignore-musl                        | DejaGnu directive |
| ignore-netbsd                      | DejaGnu directive |
| ignore-nightly                     | DejaGnu directive |
| ignore-none                        | DejaGnu directive |
| ignore-nto                         | DejaGnu directive |
| ignore-nvptx64                     | DejaGnu directive |
| ignore-nvptx64-nvidia-cuda         | DejaGnu directive |
| ignore-openbsd                     | DejaGnu directive |
| ignore-pass                        | DejaGnu directive |
| ignore-powerpc                     | DejaGnu directive |
| ignore-remote                      | DejaGnu directive |
| ignore-riscv64                     | DejaGnu directive |
| ignore-s390x                       | DejaGnu directive |
| ignore-sgx                         | DejaGnu directive |
| ignore-sparc64                     | DejaGnu directive |
| ignore-spirv                       | DejaGnu directive |
| ignore-stable                      | DejaGnu directive |
| ignore-stage1                      | DejaGnu directive |
| ignore-stage2                      | DejaGnu directive |
| ignore-test                        | DejaGnu directive |
| ignore-thumb                       | DejaGnu directive |
| ignore-thumbv8m.base-none-eabi     | DejaGnu directive |
| ignore-thumbv8m.main-none-eabi     | DejaGnu directive |
| ignore-tvos                        | DejaGnu directive |
| ignore-unix                        | DejaGnu directive |
| ignore-unknown                     | DejaGnu directive |
| ignore-uwp                         | DejaGnu directive |
| ignore-visionos                    | DejaGnu directive |
| ignore-vxworks                     | DejaGnu directive |
| ignore-wasi                        | DejaGnu directive |
| ignore-wasm                        | DejaGnu directive |
| ignore-wasm32                      | DejaGnu directive |
| ignore-wasm32-bare                 | DejaGnu directive |
| ignore-wasm64                      | DejaGnu directive |
| ignore-watchos                     | DejaGnu directive |
| ignore-windows                     | DejaGnu directive |
| ignore-windows-gnu                 | DejaGnu directive |
| ignore-windows-msvc                | DejaGnu directive |
| ignore-x32                         | DejaGnu directive |
| ignore-x86                         | DejaGnu directive |
| ignore-x86_64                      | DejaGnu directive |
| ignore-x86_64-apple-darwin         | DejaGnu directive |
| ignore-x86_64-pc-windows-gnu       | DejaGnu directive |
| ignore-x86_64-unknown-linux-gnu    | DejaGnu directive |
| only-16bit                         | DejaGnu directive |
| only-32bit                         | DejaGnu directive |
| only-64bit                         | DejaGnu directive |
| only-aarch64                       | DejaGnu directive |
| only-aarch64-unknown-linux-gnu     | DejaGnu directive |
| only-apple                         | DejaGnu directive |
| only-arm                           | DejaGnu directive |
| only-avr                           | DejaGnu directive |
| only-beta                          | DejaGnu directive |
| only-bpf                           | DejaGnu directive |
| only-cdb                           | DejaGnu directive |
| only-gnu                           | DejaGnu directive |
| only-i686-pc-windows-gnu           | DejaGnu directive |
| only-i686-pc-windows-msvc          | DejaGnu directive |
| only-ios                           | DejaGnu directive |
| only-linux                         | DejaGnu directive |
| only-loongarch64                   | DejaGnu directive |
| only-loongarch64-unknown-linux-gnu | DejaGnu directive |
| only-macos                         | DejaGnu directive |
| only-mips                          | DejaGnu directive |
| only-mips64                        | DejaGnu directive |
| only-msp430                        | DejaGnu directive |
| only-msvc                          | DejaGnu directive |
| only-nightly                       | DejaGnu directive |
| only-nvptx64                       | DejaGnu directive |
| only-powerpc                       | DejaGnu directive |
| only-riscv64                       | DejaGnu directive |
| only-s390x                         | DejaGnu directive |
| only-sparc                         | DejaGnu directive |
| only-sparc64                       | DejaGnu directive |
| only-stable                        | DejaGnu directive |
| only-thumb                         | DejaGnu directive |
| only-tvos                          | DejaGnu directive |
| only-unix                          | DejaGnu directive |
| only-visionos                      | DejaGnu directive |
| only-wasm32                        | DejaGnu directive |
| only-wasm32-bare                   | DejaGnu directive |
| only-wasm32-wasip1                 | DejaGnu directive |
| only-watchos                       | DejaGnu directive |
| only-windows                       | DejaGnu directive |
| only-windows-gnu                   | DejaGnu directive |
| only-windows-msvc                  | DejaGnu directive |
| only-x86                           | DejaGnu directive |
| only-x86_64                        | DejaGnu directive |
| only-x86_64-fortanix-unknown-sgx   | DejaGnu directive |
| only-x86_64-pc-windows-gnu         | DejaGnu directive |
| only-x86_64-pc-windows-msvc        | DejaGnu directive |
| only-x86_64-unknown-linux-gnu      | DejaGnu directive |
| needs-asm-support                  | DejaGnu directive |
| needs-deterministic-layouts        | DejaGnu directive |
| needs-dlltool                      | DejaGnu directive |
| needs-dynamic-linking              | DejaGnu directive |
| needs-force-clang-based-tests      | DejaGnu directive |
| needs-git-hash                     | DejaGnu directive |
| needs-llvm-components              | DejaGnu directive |
| needs-llvm-zstd                    | DejaGnu directive |
| needs-profiler-support             | DejaGnu directive |
| needs-relocation-model-pic         | DejaGnu directive |
| needs-run-enabled                  | DejaGnu directive |
| needs-rust-lld                     | DejaGnu directive |
| needs-sanitizer-address            | DejaGnu directive |
| needs-sanitizer-cfi                | DejaGnu directive |
| needs-sanitizer-dataflow           | DejaGnu directive |
| needs-sanitizer-hwaddress          | DejaGnu directive |
| needs-sanitizer-kcfi               | DejaGnu directive |
| needs-sanitizer-leak               | DejaGnu directive |
| needs-sanitizer-memory             | DejaGnu directive |
| needs-sanitizer-memtag             | DejaGnu directive |
| needs-sanitizer-safestack          | DejaGnu directive |
| needs-sanitizer-shadow-call-stack  | DejaGnu directive |
| needs-sanitizer-support            | DejaGnu directive |
| needs-sanitizer-thread             | DejaGnu directive |
| needs-symlink                      | DejaGnu directive |
| needs-threads                      | DejaGnu directive |
| needs-unwind                       | DejaGnu directive |
| needs-wasmtime                     | DejaGnu directive |
| needs-xray                         | DejaGnu directive |
| no-system-llvm                     | DejaGnu directive |
| min-llvm-version                   | DejaGnu directive |
| min-system-llvm-version            | DejaGnu directive |

## [Environment variable headers](#environment-variable-headers)

| Rustc Directive | DejaGnu Directive |
| --------------- | ----------------- |
| rustc-env       | DejaGnu directive |
| exec-env        | DejaGnu directive |
| unset-exec-env  | DejaGnu directive |
| unset-rustc-env | DejaGnu directive |

## [Miscellaneous headers](#miscellaneous-headers)

| Rustc Directive       | DejaGnu Directive |
| --------------------- | ----------------- |
| compile-flags         | DejaGnu directive |
| run-flags             | DejaGnu directive |
| edition               | DejaGnu directive |
| failure-status        | DejaGnu directive |
| should-fail           | DejaGnu directive |
| gate-test-X           | DejaGnu directive |
| error-pattern         | DejaGnu directive |
| incremental           | DejaGnu directive |
| no-prefer-dynamic     | DejaGnu directive |
| no-auto-check-cfg     | DejaGnu directive |
| force-host            | DejaGnu directive |
| revisions             | DejaGnu directive |
| unused-revision-names | DejaGnu directive |
| forbid-output         | DejaGnu directive |
| should-ice            | DejaGnu directive |
| known-bug             | DejaGnu directive |

## [Assembly](compiletest.md#assembly-tests) headers

| Rustc Directive | DejaGnu Directive |
| --------------- | ----------------- |
| assembly-output | DejaGnu directive |

## [Tool-specific headers](#tool-specific-headers)

| Rustc Directive | DejaGnu Directive |
| --------------- | ----------------- |
| filecheck-flags | DejaGnu directive |
| llvm-cov-flags  | DejaGnu directive |
