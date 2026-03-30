# Design Document
## Overview
This document describes the design of RusTOS, including its architecture, components, and key design decisions.
![Architecture Diagram](./assets/images/architecture-diagram.svg)

## Design decisions
- **Memory allocation**: Not all embedded systems support dynamic memory allocation, so RusTOS will avoid using it. Instead, it will rely on static memory allocation and stack-based data structures.
- **Run Trait**: To construct a task in RusTOS, users will need to provide a struct that implements the `Run` trait. This allows the user to provide parameters to the task in a safe rust way, without casting a void pointer.
- **Scheduler Singleton**: The scheduler will be implemented as a singleton, ensuring that there is only one instance of the scheduler managing all tasks in the system.
