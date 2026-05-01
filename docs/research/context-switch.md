# Context switch

> ## Summary
> This document explain what a context switch is, why it is necessary for a preemptive scheduler, and how it works.

## What is a context switch?
A context switch is the process of saving the state of all the registers (the context) of the currently running task and loading the registers for the next task to run. This is a way for the scheduler to seize control of the CPU and switch between tasks.

## Why is it necessary?
The moment a microcontroller is supposed to run more tasks than it has cores, it needs some way of managing the execution of those tasks. The RTOS solution to this problem is a preemptive scheduler. For a scheduler to be preemptive, it needs to be able to switch between tasks even before the current task has finished its execution. This makes it so a single core can run multiple tasks by switching between them.

## How does a context switch work?
A context switch consists of the following steps:
1. The scheduler decides to switch to a different task.
2. A timer interrupt or a software interrupt is triggered. (This can be either a scheduled timer event or a call to a yield function by the currently running task.)
3. The naked interrupt handler is called, which saves the context of the currently running task. This handler is only the assembly instruction to save current context. This is important because otherwise, the interrupt handler would alter the context before saving it.
4. The scheduler is called, which decides which task to run next based on its scheduling algorithm. The scheduler can be called from the interrupt handler because at this point, the context of the previous task has already been saved.
5. Back in the interrupt handler, the context of the next task is loaded, and the interrupt flag is cleared. When the interrupt handler finishes, the CPU will automatically switch to the next task.


## Sources
- [FreeRTOS Documentation](https://www.freertos.org/Documentation)
