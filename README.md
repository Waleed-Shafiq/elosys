
# Elosys

elosys is a Layer 1 blockchain that provides the strongest privacy guarantees on every single transaction. Leveraging zero-knowledge proofs (zk-SNARKs), and the highest industry standards for encryption.


## Developer Install

1. Install [Node.js LTS](https://nodejs.org/en/download/)
1. Install [Rust](https://www.rust-lang.org/learn/get-started).
1. Install [Yarn](https://classic.yarnpkg.com/en/docs/install).
1. Windows:

   1. Install the current version of Python from the [Microsoft Store package](https://www.microsoft.com/en-us/p/python-310/9pjpw5ldxlz5).
   1. Install Visual C++ Build Environment: [Visual Studio Build Tools](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools)
      (using "Visual C++ build tools" or "Desktop development with C++" workload)

   If the above steps didn't work for you, please visit [Microsoft's Node.js Guidelines for Windows](https://github.com/Microsoft/nodejs-guidelines/blob/master/windows-environment.md#compiling-native-addon-modules) for additional tips.

1. Run `yarn install` from the root directory to install packages.

   - If `yarn install` fails with an error that includes "Failed to build cmake", you may need to first install cmake. For example, on macOS:

     1. Run `brew install cmake`, you'll need cmake version 3.15 or higher.

   - If `yarn install` fails with an error that includes "Could NOT find OpenSSL", you may need to first install OpenSSL and add an environment variable. For example, on macOS:

     1. Run `brew install openssl`
     1. Run `` export OPENSSL_ROOT_DIR=`brew --prefix openssl`  ``
     1. Run `yarn install` again.

   - If `yarn install` fails with an error that includes "Error: not found: make", "make: cc: Command not found", or "make: g++: Command not found", you may need to [install a C/C++ compiler toolchain](https://github.com/nodejs/node-gyp#on-unix).

     1. On Ubuntu: `apt install build-essential`
     1. On Amazon Linux: `sudo yum groupinstall "Development Tools"`

   - If `yarn install` fails with an error that includes "Error: Could not find any Python installation to use", you may need to install Python3 (required by node-gyp). on macOS:
     1. Run `brew install python`

## Running Tests

1. To test the entire monorepo:
   1. Run `yarn test` at the root of the repository 
   1. Run `yarn test:slow` in ./elosys/ to run slow tests
   1. Run `yarn test:coverage` at the root of the repository for tests and coverage
1. To test a specific project
   1. Run `yarn test` at the root of the project
   1. Run `yarn test:watch` in ./elosys or ./elosys-cli if you want the tests to run on change
   1. Run `yarn test:coverage:html` if you want to export the coverage in an easy-to-use format (open the index.html file in the coverage folder of the project)

## Running Benchmarks and Performance Tests

1. Rust benchmarks:
   - `cargo benchmark` is a cargo alias, defined in `./.cargo/config.toml`
   1. `cargo benchmark` to run all benchmark tests
   1. `cargo benchmark -- simple` to run only benchmarks containing the text 'simple' in the name
1. Typescript benchmarks:
   1. `cd elosys`
   1. `yarn test:perf`

## Contributing Code

If you want to contribute code, you must first read [our contributing guidelines](./CONTRIBUTING.md) or risk having your pull request closed.


## Licensing

This code base and any contributions will be under the [MPL-2.0](https://www.mozilla.org/en-US/MPL/2.0/) Software License.
# Elosys
