# Dino resolver

Package managers use what is known as a **solver/package resolver** to identify exact versions of dependencies that are compatible with each other. Each solver has different properties and _may_ have different solutions. This is what makes things difficult: either we manually port npm's package solver to Rust, which takes a lot of time, or use a different solver such as Pubgrub (more on that later)

## deno_npm
This is the registry client and dependency resolver used by [Deno.](https://deno.com)

### Pros
- Deno has been battle-tested, and deno_npm has plenty of tests.
- Written in Rust, no need to call out to Node to solve packages.

### Cons
- Deeply integrated with a lot of the deno_* crates, which removes the point (this is a project for learning after all!)
- Weird Deno-related quirks, such as a [hardcoded version of @types/node](https://github.com/denoland/deno_npm/blob/8e6a3bd35249868c7266b9efca441c1bf259f0e8/src/resolution/common.rs#L152)

## Pubgrub
Used by Dart, and by extension Flutter.

### Pros
- It's fast. Like, really fast.
- Battle-tested with Flutter and Dart, which are arguably more popular than Deno.
- Great error messages built-in

### Cons
- The differences between the Pubgrub and the npm solver could use incompatibilities and subtle issues (such as one working fine, and the other using a broken version of a package)
