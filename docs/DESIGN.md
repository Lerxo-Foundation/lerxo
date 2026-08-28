# Lerxo Design

## Purpose

Lerxo is being designed as a modern general-purpose operating system with a strong emphasis on security, reliability, efficiency, and a coherent user experience.
The system should remain approachable to developers while providing the capabilities expected from a contemporary operating system.

## Core Principles

### 1. Security by Default

Security should be part of the architecture rather than an additional layer applied later.
Components should receive only the authority they require.

### 2. Clear Boundaries

The system should maintain clear boundaries between:
* Kernel components
* System services
* Drivers
* Applications
* User data

Interfaces between these components should be explicit and documented.

### 3. Memory Safety

Rust will be the primary implementation language.
Where unsafe code is required, it should be isolated, documented, and reviewed carefully.

### 4. Reliability

A failure in one component should not unnecessarily compromise unrelated components.
The system should favour predictable behaviour over unnecessary complexity.

### 5. Performance

Performance should be considered throughout the system while avoiding premature optimisation.
Correctness and measurable performance should guide optimisation decisions.

### 6. Modularity

Components should have clearly defined responsibilities and interfaces.
Where practical, components should be replaceable without requiring major changes elsewhere in the system.

## Kernel Philosophy

The kernel should provide fundamental mechanisms rather than implementing unnecessary high-level policy.
Kernel responsibilities should include:

* CPU management
* Memory management
* Scheduling
* Process management
* Interrupt handling
* Inter-process communication
* Hardware access
* System calls
* Security enforcement

Higher-level functionality should remain outside the kernel where practical.

## User Experience

Lerxo should provide a consistent environment for both command-line and graphical applications.
The system should not require users to understand the internal architecture to perform ordinary tasks.

## Compatibility

Compatibility with existing operating systems is not an initial design requirement.
Lerxo will prioritise a coherent native system interface.
Compatibility layers may be considered independently in the future.

## Hardware

The initial target is x86-64 hardware using UEFI.
Development and testing will initially take place in virtual machines.
Additional architectures and hardware platforms may be supported in the future.

## Open Source

Lerxo is developed as an open-source project.
The source code is licensed under the Apache License 2.0.
The project should remain accessible to individuals, researchers, educational institutions, and commercial organisations.

## Evolution

The design described here is an initial direction rather than a permanent specification.
Architectural decisions should be revised when implementation experience, testing, or new requirements demonstrate a better approach.
