# Telera Layout

V0.5.0

a flex-box style UI layout engine based on [clay](https://github.com/nicbarker/clay).

It could be called a wrapper, but this goes a little farther than just wrapping clay, as all the clay names and types are abstracted and renamed to be more generic. Also, the CSSCololor library is included so that the Color type can easily implement the `FromStr` trait


### currently working on:
- adding tests
- adding asserts.
- cross-compatability
    - ✅ Windows 10/11 x86
    - 🛠️ Ubuntu Linux
    - 🛠️ Omarchy (Arch) Linux
    - ❌ MacOs
    - ❌ WebAssembly (Browswer)
    - 🛠️ Arm

Develepment is tied to the development of the Clay library, with any other changes coming from bug fixes or updates to the rust languages.