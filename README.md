# Strange allocation/addressing failure

``` sh
$ rustup target add wasm32-wasi
$ cargo build --target wasm32-wasi
```

``` sh
$ wasmtime target/wasm32-wasi/debug/testcase.wasm
...
index at 131070 len 307200
index at 131071 len 307200
index at 131072 len 307200
Segmentation fault
```

``` sh
$ wavm run --abi=wasi target/wasm32-wasi/debug/testcase.wasm
...
index at 131072 len 307200
Runtime exception: wavm.outOfBoundsMemoryAccess(+1703936)
Call stack:
  wasm!target/wasm32-wasi/debug/testcase.wasm!_ZN8testcase9PlaneData3new17h2664eb193e5f086dE+379
  wasm!target/wasm32-wasi/debug/testcase.wasm!_ZN8testcase4main17h4820968c490e6e56E+58
  wasm!target/wasm32-wasi/debug/testcase.wasm!_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h1b5cf38201ff41a0E+20
  wasm!target/wasm32-wasi/debug/testcase.wasm!_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h51b9f3fe0bfdc5cdE+3
  wasm!target/wasm32-wasi/debug/testcase.wasm!_ZN3std9panicking3try7do_call17hcce8bad532002067E+7
  wasm!target/wasm32-wasi/debug/testcase.wasm!__rust_maybe_catch_panic+2
  wasm!target/wasm32-wasi/debug/testcase.wasm!_ZN3std2rt19lang_start_internal17h7161fc5134e0d17aE+93
  wasm!target/wasm32-wasi/debug/testcase.wasm!_ZN3std2rt10lang_start17hd0cc83fd85c32f30E+47
  wasm!target/wasm32-wasi/debug/testcase.wasm!__original_main+7
  wasm!target/wasm32-wasi/debug/testcase.wasm!_start+1
  thnk!C to WASM thunk!()->()+0
  host!wavm+12617510
  host!wavm+13695445
  host!wavm+13799219
  host!wavm+11656926
  host!wavm+11646680
  host!wavm+11637813
  <2 redundant frames omitted>
  host!wavm+11636211
  host!wavm+11545749
  host!/lib64/libc.so.6!__libc_start_main+237
  host!wavm+11481128

Aborted
```
