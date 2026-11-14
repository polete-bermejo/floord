# Contributing to Floord

Thanks for taking an interest in Floord!

> [!NOTE]
> I'm a beginner in Rust, and Floord is my very first Rust project. I'm still learning, so the code might not always be perfect. Suggestions, improvements, corrections and feedback are always welcome!

## Got an idea?

Don't hesitate to open an issue, no matter how small or random your idea might seem. You don't need to ask beforehand or have everything figured out.

Found a bug? Have an idea for a new feature? Think something could be done better? Just open an issue!

## Want to contribute?

Pull requests are more than welcome! Feel free to submit one even if it's something completely new or unexpected.

Just a few things to keep in mind before submitting:

- Add tests for new functionality or bug fixes.
- Make sure existing tests still pass.
- Run cargo fmt before submitting.
- Make sure the project builds successfully.

You can check everything with:

```bash
cargo fmt
cargo test -- --include-ignored
cargo build
```

Don't worry about making everything perfect. I'll review contributions and we can figure things out together.

## That's it!

I'm building Floord while learning Rust, so there's plenty of room for improvement. I'm happy to learn from other people and hear different opinions.

Whether it's a tiny bug fix, a big feature, a suggestion or just some feedback, all contributions are appreciated.

**Don't be afraid to open an issue or send a PR. Thanks for helping make Floord better!**

## Notes
- cargo test may fail intermittently on Windows machines due to inconsistencies in sleep timing.
