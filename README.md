# kfs_v2

## Requirements

qemu-system-x86_64
nightly version of rustc
cargo install bootimage

## TODO

- [ ] implement scroll support??
- [x] backspace handling
- [x] cursor support
- [ ] Debugger to show info (using keyboard driver)

## Next steps

### Processes

- [ ] kernel threads (basically processes??)
    - [ ] task scheduler (kernel scheduler with priority)
- [ ] user threads (pthreads)
    - [ ] user-level scheduler

### Other

- [ ] Filesystem
- [ ] Syscalls / Sockets / User land
- [ ] Module interface and loading (reincaranation server and service user program)
