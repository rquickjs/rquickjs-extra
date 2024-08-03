# Rquickjs Extra

This library contains modules for [rquickjs](https://github.com/DelSkayn/rquickjs) which is a high level bindings for the [QuickJS](https://bellard.org/quickjs/) JavaScript engine.

You should prefer to use modules from [AWS LLRT](https://github.com/awslabs/llrt/tree/main/llrt_modules) when they are available, this repository is an overflow for modules not yet integrated or that cannot be integrated.

## Compatibility matrix

> [!NOTE]
> Only a fraction of the Node.js APIs are supported. Below is a high level overview of partially supported APIs and modules.

|               | Node.js | Rquickjs Extra | Feature   |
| ------------- | ------- | -------------- | --------- |
| Console       | ✔︎      | ✔︎⚠️           | `console` |
| Timers        | ✔︎      | ✔︎⚠️           | `timers`  |
| URL           | ✔︎      | ✔︎⚠️           | `url`     |
| Other modules | ✔︎      | ✘              | N/A       |

_⚠️ = partially supported in Rquickjs Extra_
_⏱ = planned partial support_
_\* = Not native_
_\*\* = Use fetch instead_

## License

This library is licensed under the Apache-2.0 License. See the [LICENSE](LICENSE) file.
