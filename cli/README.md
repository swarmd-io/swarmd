# swarmd
----

`swarmd` is the CLI tool desgined to intereact with (swarmd workers)[https://swarmd.io].

## Telemetry

Telemetry is opt-in but you can desactivate it setting this environment
variable:

```bash
SWARMD_TELEMETRY_LEVEL=0
```

### Data we collect

**Crash Reports** - Crash reports collect diagnostic information when swarmd crashes and sends it to help understand why the crash occurred and what changes are needed to prevent the crash in the future.

**Error Telemetry** - Error telemetry collects information about errors that do not crash the application but are unexpected.

**Usage Data** - Usage data collects information about how features are used and perform which helps us prioritize future product improvements.

#### Usage data

This is the list of every usage we keep the track of:
