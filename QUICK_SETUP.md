# 🚀 Quick Setup Guide (5 Minutes)

This is a super-fast guide to get you running Rust code in 5 minutes!

## Step 1: Install Rust (2 minutes)

### On Linux or macOS:
Open terminal and paste this:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Press 1 and Enter when prompted.

Then run:
```bash
source $HOME/.cargo/env
```

### On Windows:
1. Go to: https://rustup.rs/
2. Download and run `rustup-init.exe`
3. Follow the instructions
4. Restart your terminal

## Step 2: Verify Installation (30 seconds)

```bash
rustc --version
cargo --version
```

You should see version numbers. If you do, you're ready! 🎉

## Step 3: Run Your First Program (1 minute)

```bash
# Navigate to this project folder
cd rust-capstone-project

# Run Hello World
cargo run
```

You should see:
```
🦀 Welcome to Rust Programming!
...
```

## Step 4: Try the Calculator (1 minute)

```bash
rustc calculator.rs
./calculator
```

Choose an operation, enter two numbers, and see the result!

## What Next?

Read the full `TOOLKIT_DOCUMENTATION.md` for:
- Detailed explanations of how everything works
- More examples and exercises
- Troubleshooting tips
- Learning resources

---

## One-Line Test (If You Already Have Rust)

```bash
cargo run
```

If this works, you're all set! 🚀

---

## Need Help?

- **Issue:** Command not found
  **Fix:** Run `source $HOME/.cargo/env` or restart terminal

- **Issue:** Compilation errors
  **Fix:** Make sure you're in the project directory with `Cargo.toml`

- **Issue:** VS Code not working
  **Fix:** Install the rust-analyzer extension from VS Code marketplace

For more help, see the [Troubleshooting section](TOOLKIT_DOCUMENTATION.md#7-common-issues--fixes) in the documentation.

---

**Total Setup Time:** ~5 minutes  
**Ready to Code:** ✅
