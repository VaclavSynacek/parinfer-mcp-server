# Migration to Git Dependency

## What Changed

This project previously included a cloned copy of `parinfer-rust` in the `parinfer-rust-lib/` directory with two lines modified to expose internal modules as public API.

**Previous approach:**
- Vendored library in `parinfer-rust-lib/` directory
- Modified `src/lib.rs` to change `mod parinfer` → `pub mod parinfer` and `mod types` → `pub mod types`
- Required manual maintenance to sync with upstream

**New approach:**
- Uses standard Cargo git dependency
- Points to fork at `https://github.com/VaclavSynacek/parinfer-rust.git`
- Branch: `expose-public-api`
- Commit: `68ef2417a9882344e6b93ead8844f8368c7a0b94`

## Changes Made

### In parinfer-rust fork:
1. Created branch `expose-public-api`
2. Modified `src/lib.rs`:
   - Line 11: `mod parinfer;` → `pub mod parinfer;`
   - Line 12: `mod types;` → `pub mod types;`
3. Pushed to `VaclavSynacek/parinfer-rust`

### In this project:
1. Updated `Cargo.toml` dependency:
   ```toml
   parinfer_rust = { git = "https://github.com/VaclavSynacek/parinfer-rust.git", branch = "expose-public-api" }
   ```
2. Removed `parinfer-rust-lib/` directory
3. Updated `Cargo.lock` to reflect git dependency

## Benefits

✅ **Standard Cargo workflow** - Uses git dependencies like any other Rust project
✅ **No vendored code** - Cleaner repository without third-party clones  
✅ **Easier maintenance** - Can update by changing branch/tag in Cargo.toml
✅ **Same functionality** - All code continues to work identically

## Future Path

**Option 1: Submit upstream PR**
Consider submitting a pull request to the main parinfer-rust repository to make these modules public. This would benefit the entire ecosystem and allow switching back to the official crate.

**Option 2: Maintain fork**
Continue using the fork if upstream doesn't accept the changes. The fork requires minimal maintenance (just the 2-line change).

## Verification

All tests pass:
```bash
$ cargo test
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

CLI functionality verified:
```bash
$ echo '(defn foo [x]
  (let [y (* x 2
    (+ y x' | ./target/debug/parinfer-cli

# Output: Correctly fixed parentheses
(defn foo [x]
  (let [y (* x 2)]
    (+ y x)))
```
