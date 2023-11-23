# Slint + SurrealismUI + Rust

This template should help get you started developing with `Slint` and `Rust`. And this template use `SurrealismUI` as default Component Library.

## Recommand IDE

VSCode

## Recommand Plugin

- Slint
- rust
- rust-analyzer

## About Slimk Commands

### Create Slint Project

create a project by selecting configuration items

```bash
# use default strategy to create a new project
> slimk create hello
# create a new project with a template
> slimk create hello --template slimk
> slimk create hello -t slimk
```
### Init an empty Slint Project

this command creates a new project but use the default strategy with no template , you will get an empty slint project

> you do not need to name the project , this way will use your root directory

```bash
> slimk init
```

### Select Templates(Native,Remote)

```bash
# native
> slimk list -n
# remote
> slimk list -r
# both
> slimk list -a
```