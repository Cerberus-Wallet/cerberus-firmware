# Cerberus Firmware

![img](https://repository-images.githubusercontent.com/180590388/968e6880-6538-11e9-9da6-4aef78157e94)

## Repository Structure

* **[`ci`](ci/)**: [Gitlab CI](https://gitlab.com/satoshilabs/cerberus/cerberus-firmware) configuration files
* **[`common/defs`](common/defs/)**: JSON coin definitions and support tables
* **[`common/protob`](common/protob/)**: Common protobuf definitions for the Cerberus protocol
* **[`common/tools`](common/tools/)**: Tools for managing coin definitions and related data
* **[`core`](core/)**: Cerberus Core, firmware implementation for Cerberus T
* **[`crypto`](crypto/)**: Stand-alone cryptography library used by both Cerberus Core and the Cerberus One firmware
* **[`docs`](docs/)**: Assorted documentation
* **[`legacy`](legacy/)**: Cerberus One firmware implementation
* **[`python`](python/)**: Python [client library](https://pypi.org/project/cerberus) and the `cerberusctl` command
* **[`storage`](storage/)**: NORCOW storage implementation used by both Cerberus Core and the Cerberus One firmware
* **[`tests`](tests/)**: Firmware unit test suite
* **[`tools`](tools/)**: Miscellaneous build and helper scripts
* **[`vendor`](vendor/)**: Submodules for external dependencies


## Contribute

See [CONTRIBUTING.md](docs/misc/contributing.md).

Using [Conventional Commits](COMMITS.md) is strongly recommended and might be enforced in future.

Also please have a look at the docs, either in the `docs` folder or at  [docs.cerberus.uraanai.com](https://docs.cerberus.uraanai.com) before contributing. The [misc](docs/misc/index.md) chapter should be read in particular because it contains some useful assorted knowledge.

## Security vulnerability disclosure

Please report suspected security vulnerabilities in private to [security@satoshilabs.com](mailto:security@satoshilabs.com), also see [the disclosure section on the Cerberus.io website](https://cerberus.uraanai.com/support/a/how-to-report-a-security-issue). Please do NOT create publicly viewable issues for suspected security vulnerabilities.

## Documentation

See the `docs` folder or visit [docs.cerberus.uraanai.com](https://docs.cerberus.uraanai.com).
