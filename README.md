# Telera Layout

V0.14.0

a flex-box style UI layout engine based on [clay](https://github.com/nicbarker/clay) v0.14.

It could be called a wrapper, but this goes a little farther than just wrapping clay, as all the clay names and types are abstracted and renamed to be more generic. Also, the `csscolorparser` library is included so that the `Color` type can implement the `FromStr` trait.

### platform status

- ✅ Windows 10/11 x86
- 🛠️ Ubuntu Linux
- 🛠️ Omarchy (Arch) Linux
- ❌ MacOS
- ❌ WebAssembly (browser)
- 🛠️ Arm

Development tracks the clay library; other changes come from bug fixes or Rust language updates.