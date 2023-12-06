# swarmd_local_runtime

It's a runtime based on deno in which the only purpose is to reproduce the
behavior of the Edge runtime while working locally.

To do that we re-use the already implemented Web APIs by `deno`.

Some shims are present to reproduce the behavior of the Swarmd runtime on the
cloud even if the implementation is not the same.
