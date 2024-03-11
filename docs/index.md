# Cerberus Firmware documentation

_This documentation can also be found at [docs.cerberus.uraanai.com](https://docs.cerberus.uraanai.com) where it is available in a HTML-built version compiled using [mdBook](https://github.com/rust-lang/mdBook)._

Welcome to the Cerberus Firmware repository. This repository is so called _monorepo_, it contains several different yet very related projects that together form the Cerberus Firmware ecosystem.

## Repository Structure

* **[`ci`](https://github.com/Cerberus-Wallet/cerberus-firmware/tree/master/ci/)**: [Gitlab CI](https://gitlab.com/satoshilabs/cerberus/cerberus-firmware) configuration files
* **[`common/defs`](https://github.com/Cerberus-Wallet/cerberus-firmware/tree/master/common/defs/)**: JSON coin definitions and support tables
* **[`common/protob`](https://github.com/Cerberus-Wallet/cerberus-firmware/tree/master/common/protob/)**: Common protobuf definitions for the Cerberus protocol
* **[`common/tools`](https://github.com/Cerberus-Wallet/cerberus-firmware/tree/master/common/tools/)**: Tools for managing coin definitions and related data
* **[`core`](https://github.com/Cerberus-Wallet/cerberus-firmware/tree/master/core/)**: Cerberus Core, firmware implementation for Cerberus T
* **[`crypto`](https://github.com/Cerberus-Wallet/cerberus-firmware/tree/master/crypto/)**: Stand-alone cryptography library used by both Cerberus Core and the Cerberus One firmware
* **[`docs`](https://github.com/Cerberus-Wallet/cerberus-firmware/tree/master/docs/)**: Assorted documentation
* **[`legacy`](https://github.com/Cerberus-Wallet/cerberus-firmware/tree/master/legacy/)**: Cerberus One firmware implementation
* **[`python`](https://github.com/Cerberus-Wallet/cerberus-firmware/tree/master/python/)**: Python [client library](https://pypi.org/project/cerberus) and the `cerberusctl` command
* **[`storage`](https://github.com/Cerberus-Wallet/cerberus-firmware/tree/master/storage/)**: NORCOW storage implementation used by both Cerberus Core and the Cerberus One firmware
* **[`tests`](https://github.com/Cerberus-Wallet/cerberus-firmware/tree/master/tests/)**: Firmware unit test suite
* **[`tools`](https://github.com/Cerberus-Wallet/cerberus-firmware/tree/master/tools/)**: Miscellaneous build and helper scripts
* **[`vendor`](https://github.com/Cerberus-Wallet/cerberus-firmware/tree/master/vendor/)**: Submodules for external dependencies


## Contribute

See [CONTRIBUTING.md](https://github.com/Cerberus-Wallet/cerberus-firmware/tree/master/CONTRIBUTING.md).

Also please have a look at the docs, either in the `docs` folder or at  [docs.cerberus.uraanai.com](https://docs.cerberus.uraanai.com) before contributing. The [misc](misc/index.md) chapter should be read in particular because it contains some useful assorted knowledge.

## Security vulnerability disclosure

Please report suspected security vulnerabilities in private to [security@satoshilabs.com](mailto:security@satoshilabs.com), also see [the disclosure section on the cerberus.uraanai.com website](https://cerberus.uraanai.com/security/). Please do NOT create publicly viewable issues for suspected security vulnerabilities.

## Note on terminology

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://tools.ietf.org/html/rfc2119).
