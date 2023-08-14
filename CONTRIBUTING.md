# Contribution guidelines


## Cutting a new release

1. run `unclog release vX.Y.Z`
2. run `unclog build > CHANGELOG.md`
3. run `git add CHANGELOG.md && git add .changelog`
4. bump the `Cargo.toml` version number
