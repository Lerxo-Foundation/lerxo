# Lerxo Development

This document describes the development environment, workflow, and testing practices used by the Lerxo project.

## Development Environment

Lerxo is primarily developed using:

* Rust
* Cargo
* Git
* GitHub
* QEMU for virtualised system testing

The initial development target is x86-64.

## Development Workflow

Development follows a simple cycle:
1. Design the component or change.
2. Implement the change.
3. Build and check the project.
4. Test in a virtual machine where applicable.
5. Review the implementation.
6. Commit the change.
7. Update documentation where necessary.

## Testing

Early operating-system development should be performed in a virtual machine whenever possible.
QEMU will be the initial virtualisation environment used to test Lerxo.
Physical hardware should not be required for ordinary kernel development.

## Source Control

Git is used for source control.
The `main` branch represents the primary development branch.
Changes should be kept focused and accompanied by clear commit messages.

## Releases

Stable versions of Lerxo will be distributed through GitHub Releases.
Release versions will follow semantic versioning where appropriate:

```text
MAJOR.MINOR.PATCH
```

For example:

```text
0.1.0
```

## Dependencies

Dependencies should be introduced only when they provide clear value.
Low-level components should minimise unnecessary dependencies and carefully consider the security and maintenance implications of each dependency.

## Documentation

Technical decisions should be documented when they affect the architecture or public interfaces of Lerxo.
Documentation should be kept alongside the source code and updated as the implementation changes.

## Development Status

Lerxo is currently in pre-alpha development.
The development process will become more formal as the project gains contributors and approaches its first public release.
