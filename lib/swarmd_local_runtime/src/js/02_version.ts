// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
const primordials = globalThis.__bootstrap.primordials;
const { ObjectFreeze } = primordials;

interface Version {
  deno: string;
  v8: string;
  typescript: string;
}

const version: Version = {
  deno: "",
  v8: "",
  typescript: "",
};

function setVersions(
  denoVersion: string,
  v8Version: string,
  tsVersion: string,
) {
  version.deno = `swarmd-${denoVersion}`;
  version.v8 = v8Version;
  version.typescript = tsVersion;

  ObjectFreeze(version);
}

export { setVersions, version };
