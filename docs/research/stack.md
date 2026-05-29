# Creating and using a static stack
## Introduction
In a program, the stack is used to store local data in a first-in, last-out (FILO) manner. In an RTOS, each task keeps track of its own local data, and thus, each task needs its own stack.

### Goal of this research
The goal of this research is to understand how to create a static stack in Rust, so that it can be used for the tasks in RusTOS. This is necessary because RusTOS does not use a heap, so all memory must be allocated statically.

## Structure of a stack
A stack in RusTOS is simply an array of bytes. This array cannot be created on the main stack, because it needs to be preserved across context switches. Since RusTOS does not use a heap, the stack must be created in static memory. Finally, the way the CPU keeps track of the stack is through the stack pointer. In the RISC-V architecture, this pointer steps through the stack in 16-byte increments, so the stack must be aligned to 16 bytes. Combining all these requirements, we can define a stack as follows:

```rust
#[repr(align(16))]
struct Stack {
    data: [u8; STACK_SIZE],
}

#[link_section = ".bss"] // This section is used for uninitialized static variables, which is what we want for our stack.
static mut STACK_INSTANCE: Stack = Stack {
    data: [0b; STACK_SIZE],
};
```

## Using the stack
In order to use a static stack, we need to change the stack pointer register to point to the bottom of the stack. This register is not accessible from regular Rust code, so we need to use inline assembly to set it. This might look something like this:

```rust
unsafe {
    core::arch::asm!(
        "
        la t0, {stack_addr}
        li t1, {stack_size}
        add t0, t0, t1 # Add the size of the stack to get the address of the bottom of the stack.
        mv sp, t0 # Set the stack pointer to the resulting address.
        ",
        stack_addr = sym STACK_INSTANCE,
        stack_size = const STACK_SIZE,
    );
}
```
The code after this block can now use the custom stack as normal, and the CPU will automatically manage the stack pointer as it pushes and pops data from the stack.

## Unsafe Rust
Using inline assembly in Rust code is inherently unsafe, but the reason this code specifically is unsafe is because it changes the stack pointer. If the stack pointer is not returned to point to the original stack before the function returns, the program will try to clean up data from the wrong stack, which can lead to undefined behavior.

## Sources
- [Rust Embedded Book](https://docs.rust-embedded.org/embedonomicon/memory-layout.html)
- [.bss section](https://en.wikipedia.org/wiki/.bss)
