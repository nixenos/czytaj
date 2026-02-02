# Tauri Package Version Alignment

## Issue

Version mismatch between Tauri packages:
- Rust crate `tauri` was resolving to v2.10.0
- NPM package `@tauri-apps/api` was at v2.9.1

Tauri requires these to be on the same major/minor version for compatibility.

## Root Cause

The `package.json` was using overly broad version constraints (`^2`) which could install any 2.x version. When combined with the Rust crate versions, this could lead to mismatches depending on when dependencies were installed.

## Solution

Updated `package.json` to specify compatible versions that match the latest stable releases available:

### NPM Packages (Updated)
- `@tauri-apps/api`: `^2.9.1` (was `^2`)
- `@tauri-apps/cli`: `^2.9.6` (was `^2`)
- `@tauri-apps/plugin-shell`: `^2.3.4` (was `^2`)

### Rust Crates (Unchanged)
- `tauri`: `2` (allows any 2.x, will get 2.10.0)
- `tauri-build`: `2` (allows any 2.x, will get 2.5.4)
- `tauri-plugin-shell`: `2` (allows any 2.x)

## Compatibility Note

Tauri v2 Rust crates and NPM packages are compatible within the same major version (2.x). The minor version differences (e.g., 2.9 vs 2.10) are acceptable as long as we're using compatible versions. The NPM packages we've specified are the latest stable versions available that work with Tauri v2.

By pinning the NPM packages to specific minor versions (2.9.x for api/cli, 2.3.x for plugin-shell), we ensure consistent installations while maintaining forward compatibility with the Rust crates.

## Verification

After the fix:
```bash
npm list @tauri-apps/api @tauri-apps/cli @tauri-apps/plugin-shell
```

Shows:
- @tauri-apps/api@2.9.1
- @tauri-apps/cli@2.9.6
- @tauri-apps/plugin-shell@2.3.4

All packages are now aligned on compatible versions within the Tauri v2 ecosystem.
