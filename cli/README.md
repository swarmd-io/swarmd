# swarmd
----

`swarmd` is the CLI tool desgined to intereact with (swarmd workers)[https://swarmd.io].

## Telemetry

Telemetry for errors and crash is opt-in but you can desactivate it setting this environment
variable:

// TODO(@miaxos)
```bash
SWARMD_TELEMETRY_LEVEL=0
```

### Data we collect

**Crash Reports** - Crash reports collect diagnostic information when swarmd crashes and sends it to help understand why the crash occurred and what changes are needed to prevent the crash in the future.

**Error Telemetry** - Error telemetry collects information about errors that do not crash the application but are unexpected.
