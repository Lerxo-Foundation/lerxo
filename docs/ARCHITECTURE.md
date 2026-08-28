# Lerxo Architecture

## Overview

Lerxo is designed as a modular operating system built around a small privileged kernel and a collection of system components with clearly defined interfaces.

The architecture prioritises isolation, reliability, and well-defined boundaries between components.

## System Structure

```text
Applications
     |
     v
Lerxo System APIs
     |
     v
Inter-Process Communication
     |
     +-------------------+
     |                   |
     v                   v
System Services       Kernel
                         |
             +-----------+-----------+
             |           |           |
             v           v           v
          Memory     Scheduling     IPC
          Manager
             |
             v
          Hardware
```

## Kernel

The kernel provides the fundamental mechanisms required to operate the system.

Its responsibilities include:

* CPU and architecture management
* Memory management
* Scheduling
* Process and thread management
* Interrupt handling
* System calls
* Inter-process communication
* Hardware access where required
* Security boundaries

The kernel should provide mechanisms rather than unnecessarily implementing higher-level policy.

## System Services

Higher-level functionality will be implemented through system services where practical.

Potential services include:

* Device management
* Filesystem services
* Networking
* Graphics
* Input
* System configuration
* Logging
* Package management

Services should communicate through explicit interfaces rather than directly accessing unrelated components.

## User Applications

Applications execute outside the kernel and interact with the system through documented system interfaces.

Applications should not receive direct access to kernel memory or hardware unless explicitly authorised.

## Processes

Each process will have its own virtual address space and defined access to system resources.

Processes will communicate using controlled mechanisms provided by the operating system.

## Security Model

Security will be based on isolation, least privilege, and explicit authority.

Where practical, access to system resources will be represented by capabilities or handles rather than unrestricted global access.

## Memory Management

Lerxo will use virtual memory to provide isolation between processes and between user and kernel space.

The memory subsystem will provide:

* Physical memory management
* Virtual address spaces
* Page allocation
* Kernel heap allocation
* Process memory isolation

## Scheduling

The kernel will provide preemptive multitasking.

The scheduler will manage execution of kernel threads and user processes while maintaining isolation between processes.

The initial scheduler will prioritise correctness and predictability. Performance optimisations will be introduced as the system develops.

## Storage

Storage will be exposed through a common filesystem interface.

The architecture is intended to allow multiple filesystem implementations without requiring applications to understand the underlying storage technology.

## Networking

Networking will be separated into hardware drivers, network protocols, and user-facing networking interfaces.

The planned network stack includes:

* Ethernet
* IPv4
* IPv6
* UDP
* TCP
* DNS

## Graphics

The graphics architecture will separate hardware access from the user-facing window system.

The planned graphical stack consists of:

```text
Applications
     |
     v
Window System
     |
     v
Compositor
     |
     v
Graphics Subsystem
     |
     v
Hardware
```

## Hardware

Hardware-specific functionality will be isolated behind standard interfaces wherever possible.

The initial target architecture is x86-64.

UEFI will be the initial boot environment.

## Design Principles

### Isolation

Components should have only the access they require.

### Explicit Interfaces

Communication between components should occur through documented interfaces.

### Minimal Privilege

Privileged operations should be restricted to the components that require them.

### Simplicity

The architecture should remain understandable as the system develops.

### Replaceability

Major components should be replaceable without requiring unrelated parts of the system to be rewritten.

## Current Status

This document describes the planned architecture. It will evolve as implementation and testing reveal better approaches.
