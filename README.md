# RusTOS
## Goal
RusTOS is a real-time operating system (RTOS) written in Rust. The goal of this project is to create a simple and efficient RTOS that can be used for embedded systems. The RTOS should be able to be used as a library that can be easily integrated into existing projects. There will be example applications demonstrating the features implemented in the RTOS, as well as one large example application that will be developed alongside the RTOS to demonstrate its capabilities.

> [!NOTE]
> The RTOS will be developed with the ESP32-C6 microcontroller in mind, but it should be designed to be portable to other platforms as well.

## Milestones
 - [ ] Tasks
 - [ ] Scheduler
 - [ ] Mutex
 - [ ] Queues
 - [ ] Timers
 - [ ] Networking

 TODO: Research rtos features.


## Similar Projects
 - [Embassy](https://embassy.dev/) - An async embedded framework for Rust. It uses the native async/await syntax and provides a high-level API for embedded development. It is designed to be portable across different platforms and architectures.
 - [esp-rtos](https://crates.io/crates/esp-rtos) - This crate is provided by Espressif to be used in combination with their other ESP crates.
