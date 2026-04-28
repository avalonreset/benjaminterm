---
type: concept
title: "Release Workflow Tax"
created: 2026-04-28
updated: 2026-04-28
status: active
tags:
  - ci
  - release
  - tech-debt
related:
  - "[[Cross-Platform Release Strategy]]"
  - "[[v2.0.0 Rebrand Release]]"
  - "[[Packaging and CI]]"
---

# Release Workflow Tax

The release workflow at `.github/workflows/benterm-release.yml` works, but pays an avoidable wall-clock and recovery-cost tax. Discovered during the v2.0.0 ship: ~52 minutes from first tag push to release-live, with ~17 minutes of that wasted to a single-platform code-bug failure that should have been a surgical re-run.

This page captures the design issues and the optimization punch list. None of these are blocking; all are worth landing before the next non-trivial release.

## Issue 1 — Cold Compile Every Run

The release workflow does not configure `sccache`, cargo registry caching, or any other compiler cache between runs. Every release is a from-zero compile of the entire crate graph (~700 deps including heavy native libs: cairo, freetype, harfbuzz, openssl-sys, ring).

- **Impact**: ~25 minutes wall clock per platform per run.
- **Comparison**: the per-distro `gen_*.yml` workflows (PR / continuous validation builds) DO use `mozilla-actions/sccache-action@v0.0.9`. The release workflow doesn't.
- **Fix**: add the sccache action and the standard `SCCACHE_GHA_ENABLED=true` + `RUSTC_WRAPPER=sccache` env vars to each platform job. Estimated ~10 lines of YAML.
- **Expected gain**: subsequent runs drop from ~25 min to ~10 min once the cache warms.

## Issue 2 — `prepare-release` Deletes The Release Every Run

`benterm-release.yml` lines 51-53:

```yaml
if gh release view "$tag" --repo "$GITHUB_REPOSITORY" >/dev/null 2>&1; then
  gh release delete "$tag" --repo "$GITHUB_REPOSITORY" --cleanup-tag --yes
fi
```

This deletes the existing GitHub release (and its assets) at the start of every triggered run.

- **Impact**: when a single platform job fails due to a code bug, fixing requires a new commit, which requires a re-tag, which triggers a new run, which deletes the existing partial assets. Linux + Windows artifacts that already succeeded get wiped along with the failed macOS state.
- **Why it's there**: ensures clean state, no half-stale assets from a previous attempt.
- **Better pattern (idempotent prepare-release)**: skip-if-exists rather than delete-then-create. Each platform job already calls `gh release upload --clobber`, so re-uploading replaces specific assets cleanly without touching the others. A code-bug failure becomes "push fix → manually re-run only the failed platform job → it overwrites its asset → publish." Linux + Windows untouched.
- **Estimated cost**: ~5 lines of YAML to gate the delete behind a `force_recreate` workflow input that defaults to `false`.

## Issue 3 — macOS Universal Build Serializes 8 Cargo Invocations

`benterm-release.yml` lines 149-153:

```bash
for bin in wezterm wezterm-gui wezterm-mux-server strip-ansi-escapes; do
  cargo build --target x86_64-apple-darwin -p "$bin" --release
  cargo build --target aarch64-apple-darwin -p "$bin" --release
done
```

Eight sequential `cargo build` calls. Cargo can parallelize across packages within a single invocation; the loop forces serial.

- **Impact**: macOS becomes the wall-clock bottleneck (~24 min vs. ~22 min for Windows).
- **Fix variant A**: collapse the inner loop to one cargo call per arch.

  ```bash
  for target in x86_64-apple-darwin aarch64-apple-darwin; do
    cargo build --target "$target" --release \
      -p wezterm -p wezterm-gui -p wezterm-mux-server -p strip-ansi-escapes
  done
  ```

  Saves ~3-5 min by parallelizing within cargo.

- **Fix variant B (better)**: split macOS into two separate jobs (`macos-x86_64` and `macos-aarch64`) running in parallel on two runners. Lipo together in a third small job.

  Saves ~10 min. Trade-off: ships two macOS .zip artifacts (Intel + Apple Silicon) instead of one universal. Apple is deprecating Intel macOS; arm64-only is increasingly defensible.

## Issue 4 — Job Status Is Not The Same As Asset Availability

A platform job's `Upload release assets` step typically completes well before the job itself transitions to `completed` status. The job runs cleanup steps (post-checkout, post-Node, etc.) after the upload, during which the job-level status remains `in_progress`.

- **Trap**: status checks that key off `gh run view --json jobs --jq '.jobs[].status'` miss the fact that the artifact is already on the draft release.
- **Cost during v2.0.0 ship**: a status check confidently reported "the installer doesn't exist yet" while it had been uploaded ~2 minutes earlier. Caused unnecessary user wait and a force-push that would have been wiser had we already-grabbed the local copy.
- **Fix**: ground-truth check during a release ship is `gh release view "$TAG" --json assets --jq '.assets[].name'`. Use this in addition to (or instead of) job status when the question is "can we install yet?".

## Issue 5 — Code Signing Is Configured But Not Active

`ci/package-benterm-macos.sh` includes a code-signing block gated on `MACOS_TEAM_ID`. That secret is not currently set in the repo's GitHub Actions secrets, so the block is a no-op. Released macOS artifacts are unsigned and unnotarized; users must `xattr -dr com.apple.quarantine` after download to launch.

- **Impact on ship**: the workflow lies about its intent. The block is dead code that suggests signing is happening when it isn't.
- **Fix options**:
  - (a) Set up Apple Developer credentials, populate the secrets, sign for real.
  - (b) Remove the dead block entirely until signing is real, document in [[Cross-Platform Release Strategy]].
- **Status**: deferred; existing tracker is the active-thread item "Decide whether macOS release should be signed/notarized before public announcement."

## Recovery Pattern (Until Fixes Land)

When a single platform job fails on a release run:

1. **Don't cancel the in-progress jobs** that haven't failed yet. Let them complete and upload to the draft release.
2. **Download the successful artifacts locally** before doing anything that could trigger a re-run. `gh release download "$TAG" -p '...'` works on draft releases for repo collaborators.
3. **Then** push the fix and re-tag. The new run's `prepare-release` will wipe the GitHub release, but you have local copies.
4. After the fresh run lands, the locally-downloaded artifacts and the new release artifacts will be byte-identical for unchanged platforms (same code, same compile flags). For the platform you fixed, the new run produces a corrected artifact.

This cost us a Windows installer's worth of wait during the v2.0.0 ship — captured in the [[v2.0.0 Rebrand Release]] log.

## Optimization Punch List (Ordered by ROI)

1. ⭐ Wire `sccache` into `benterm-release.yml` — biggest single-line-of-YAML win. Drops subsequent builds from ~25 min to ~10.
2. ⭐ Make `prepare-release` idempotent. Enables surgical recovery from single-platform failures.
3. Collapse macOS for-loop to one-cargo-per-arch. ~3-5 min saved.
4. Split macOS into parallel arch-specific jobs. ~10 min saved, two artifacts shipped.
5. Decide on macOS signing (real or remove the block).
6. Document the "asset availability ≠ job status" rule in `RELEASING_BENTERM.md`.
