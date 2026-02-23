# WCS — WoW CLI Synchronizer

<!--toc:start-->
- [WCS — WoW CLI Synchronizer](#wcs-wow-cli-synchronizer)
  - [Philosophy](#philosophy)
  - [Current Features](#current-features)
    - [List](#list)
  - [Planned Features](#planned-features)
    - [Install](#install)
    - [Remove](#remove)
    - [Update](#update)
    - [Import](#import)
    - [Diff](#diff)
  - [Considered Features](#considered-features)
    - [WTF Folder Sync](#wtf-folder-sync)
<!--toc:end-->

> A minimal, fast command-line tool for synchronizing World of Warcraft addons.
> No bloat. No UI. Just control.

WCS is a lightweight CLI utility written in Rust for managing and sharing World of Warcraft addon setups. Think `npm` or `cargo` but for your WoW addons — reproducible, scriptable, and version-controlled.

---

## Philosophy

- **Minimal** — does one thing well, no feature creep
- **Deterministic** — lockfiles mean the same setup every time
- **Git-friendly** — commit your lockfile, share your setup
- **No Electron, only Rust** — fast, native, no runtime overhead
- **Client-side only** — no accounts, no telemetry, no servers
- **Dotfile configuration** — fits right into your existing workflow

---

## Current Features

### List

Scan your World of Warcraft installation and list all currently installed addons.

```bash
wcs list
```

---

## Planned Features

### Install

Install a single addon or restore from a lockfile. Pin a specific version to freeze it from automatic updates.

```bash
wcs install <addon> [--provider <provider>] [--version <version>]
```

### Remove

Remove a single addon.

```bash
wcs remove <addon>
```

### Update

Update a single addon, or all of them if no name is given.

```bash
wcs update [--name <addon>]
```

### Import

Import a full addon setup from a lockfile.

```bash
wcs import <lockfile>
```

### Diff

Compare your lockfile against what's actually installed.

```bash
wcs diff
```

---

## Considered Features

### WTF Folder Sync

If there is interest.
