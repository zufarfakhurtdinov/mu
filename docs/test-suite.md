# Test suite

The test suite should treat Mu as a black-box executable. It should run a built Mu process through its public command-line interface and verify only observable behavior, such as exit codes, standard output, standard error, and filesystem changes. It must not depend on Mu's implementation language or internal APIs, so the same suite can test alternative implementations.

Each test should run in an isolated, reproducible environment with explicitly controlled inputs and dependencies. See Bazel's [test hermeticity guidance](https://bazel.build/reference/test-encyclopedia) for the principles behind keeping tests independent from undeclared host state.
