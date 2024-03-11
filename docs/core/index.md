# Cerberus Core

Cerberus Core is the second-gen firmware running on Cerberus devices. It currently runs on Cerberus T and Cerberus Safe 3, but it might be used on Cerberus One in the future as well (see issue [#24](https://github.com/Cerberus-Wallet/cerberus-firmware/issues/24)).

Cerberus Core is part of the cerberus-firmware monorepo to be found on [GitHub](https://github.com/Cerberus-Wallet/cerberus-firmware), in the `core` subdirectory.

Cerberus Core uses [MicroPython](https://github.com/micropython/micropython), it is a Python implementation for embedded systems, which allows us to have an application layer in Python, which makes the code significantly more readable and sustainable. This is what you find in the `src` folder.

Not everything is in Python though, we need to use C occasionally, usually for performance reasons. That is what `embed/extmod` is for. It extends MicroPython's modules with a number of our owns and serves as a bridge between C and Python codebase. Related to that, `mocks` contain Python mocks of those functions to improve readability and IDE functioning.

Where appropriate, we also use Rust. For example, all UI components and animations are implemented in `embed/rust`. Similarly to C bindings, you can find Python mocks for the Rust functions in `mocks` directory. Developing new features in Rust is preferred in the future.

## Boot

Module `src/main.py` is the first one to be invoked in MicroPython. It starts the USB, initializes the wire codec and boots applications (see [Apps](apps.md)).
