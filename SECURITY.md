# Security

Security is a fundamental design objective of Lerxo.
Lerxo is being designed with isolation, least privilege, memory safety, and explicit authority as core principles.

## Security Principles

### Memory Isolation

User processes must not be able to access kernel memory or the memory belonging to other processes without explicit authorisation.

### Least Privilege

Components should receive only the permissions required to perform their functions.
Privileged operations should be restricted to trusted system components.

### Process Isolation

Applications should execute in isolated address spaces.
A failure in one application should not normally compromise other applications or the kernel.

### Explicit Authority

Access to sensitive system resources should be explicitly granted.
Where appropriate, Lerxo will use handles or capabilities to represent access to resources.

### Memory Safety

Lerxo is primarily written in Rust.
Unsafe Rust will be restricted to areas where direct interaction with hardware or other low-level mechanisms requires it.
Unsafe code should be kept small, documented, and reviewed carefully.

### Secure Interfaces

System calls and inter-process communication interfaces will validate inputs and enforce access controls.
User-provided data must never be trusted simply because it crosses a system boundary.

### Secure Updates

The eventual update mechanism should provide cryptographic verification of system components before installation.

## Security-Critical Components

Particular care will be taken with:

* Boot code
* Memory management
* Process isolation
* System calls
* Inter-process communication
* Device drivers
* Filesystems
* Networking
* Authentication
* Cryptographic functionality
* System updates

## Vulnerability Reporting

Security vulnerabilities should not be publicly disclosed before the project has had an opportunity to investigate and address them.
A dedicated security reporting process will be established as the project approaches its first public release.

## Development

Early versions of Lerxo are experimental and should not be trusted with sensitive information or important data.
Security decisions and implementation details will be documented as the project develops.
