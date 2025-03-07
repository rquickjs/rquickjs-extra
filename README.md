# Rquickjs Extra

[![github](https://img.shields.io/badge/github-rquickjs/rquickjs-extra.svg?style=for-the-badge&logo=github)](https://github.com/rquickjs/rquickjs-extra)
[![crates](https://img.shields.io/crates/v/rquickjs-extra.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/rquickjs-extra)

This library contains modules for [rquickjs](https://github.com/DelSkayn/rquickjs) which is a high level bindings for the [QuickJS](https://bellard.org/quickjs/) JavaScript engine.

You should prefer to use modules from [AWS LLRT](https://github.com/awslabs/llrt/tree/main/llrt_modules) when they are available, this repository is an overflow for modules not yet integrated or that cannot be integrated.

## Compatibility matrix

> [!NOTE]
> Only a fraction of the Node.js APIs are supported. Below is a high level overview of partially supported APIs and modules.

|               | Node.js | Rquickjs Extra | Feature   |
| ------------- | ------- | -------------- | --------- |
| Console       | ✔︎      | ✔︎⚠️           | `console` |
| OS            | ✔︎      | ✔︎⚠️           | `os`      |
| Timers        | ✔︎      | ✔︎⚠️           | `timers`  |
| URL           | ✔︎      | ✔︎⚠️           | `url`     |
| Sqlite        | ⏱       | ✔︎⚠️           | `sqlite`  |
| Other modules | ✔︎      | ✘              | N/A       |

_⚠️ = partially supported in Rquickjs Extra_
_⏱ = planned partial support_
_\* = Not native_
_\*\* = Use fetch instead_

## License

This library is licensed under the Apache-2.0 License. See the [LICENSE](LICENSE) file.
