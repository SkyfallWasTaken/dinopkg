# Dino

Dino is a brand new package manager for Node.js, written in Rust. It aims to be a drop-in replacement for npm that's faster and safer.

## Current commands

### `init`

`dino init` intelligently uses your working environment and Git config to create a `package.json` with sane defaults.

![image](https://github.com/user-attachments/assets/7da8d0d1-4ac0-45b6-9c31-98a877b09649)

In the above image, the `repository` was automatically picked up on by Dino! (So was the default `name`, but it was changed manually)

### `run`

`dino run` runs scripts in your `package.json` file!

![image](https://github.com/user-attachments/assets/abaf46ca-9c66-45b0-a019-4a9eeed0abb0)

If no arguments are provided, it instead lists the scripts in the `package.json` file.

![image](https://github.com/user-attachments/assets/be13b45a-dbb4-4144-b61e-65d568ef1be2)

### `install`

`dino install` currently finds dependencies of the package listed.

![image](https://github.com/user-attachments/assets/29085747-15ed-413c-a599-f56441998562)

## Notes on package resolvers
I wrote some notes on npm package resolvers, available [here.](https://github.com/SkyfallWasTaken/dinopkg/blob/master/RESOLVER_NOTES.md)

---

_ © 2024 Mahad Kalam_

_Licensed under MIT License_
