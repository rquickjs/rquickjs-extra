# rquickjs extra

This library contains modules for [rquickjs](https://github.com/DelSkayn/rquickjs) which is a high level bindings for the [QuickJS](https://bellard.org/quickjs/) JavaScript engine.

Built upon the great work done in [AWS LLRT](https://github.com/awslabs/llrt).

## Compatibility matrix

> [!NOTE]
> Only a fraction of the Node.js APIs are supported. Below is a high level overview of partially supported APIs and modules.

|               | Node.js | Rquickjs |
| ------------- | ------- | -------- |
| child_process | ✔︎      | ✔︎⚠️     |
| fs/promises   | ✔︎      | ✔︎       |
| fs            | ✔︎      | ✘⏱       |
| Other modules | ✔︎      | ✘        |

_⚠️ = partially supported in Rquickjs_
_⏱ = planned partial support_
_\* = Not native_
_\*\* = Use fetch instead_

## License

This library is licensed under the Apache-2.0 License. See the [LICENSE](LICENSE) file.
