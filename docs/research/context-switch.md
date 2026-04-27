# Context switch

> ## Summary
> This document explain what a context switch is, why it is necessary for a preemptive scheduler, and how it works.

## What is a context switch?
A context switch is the process of saving the state of all the registers (the context) of the currently running task and loading the registers for the next task to run. This is a way for the scheduler to seize control of the CPU and switch between tasks.

## Why is it necessary?
The moment a microcontroller is supposed to run more tasks than it has cores, it needs some way of managing the execution of those tasks. The RTOS solution to this problem is a preemptive scheduler. For a scheduler to be preemptive, it needs to be able to switch between tasks even before the current task has finished its execution. This makes it so a single core can run multiple tasks by switching between them.

## How does a context switch work?


## Sources
- [FreeRTOS Documentation](https://www.freertos.org/Documentation)
