# ui_sim

user interface simulator for a handheld multilingual communication device


## What is this about?

The the point is to prototype user interface ideas for a handheld communication
device, with an emphasis on multilingual keyboard layouts.

The approach is to simulate a keyboard, LCD, microkernel, and graphics server
to enable work on user facing software that could be conveniently ported to a
real microkernel, on real hardware, once those are ready.


## Repository Tour

- **mktcp** (rust): Simulated microkernel using message passing over local tcp.
  Keyboard and graphics use browser APIs with javascript. Everything else runs
  as native-compiled rust binary.

- **mkwasm** (rust): Simulated microkernel using message passing over
  webassembly shared memory IPC. Keyboard and graphics use browser APIs with
  javascript. Everything else runs as cross-compiled webassembly rust library.

- **www** (html/css/js): Static web pages with user interface components
  including simulated keyboard hardware, simulated LCD, graphics toolkit rom
  editor, sprite editor, and debug monitor.
