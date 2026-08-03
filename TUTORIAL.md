## Important!

**I wrote this at 2am and without testing, this is just what I remember doing when I was doing iOS mobile. So take this with a grain of salt before this message disappears :D (that means i tested this and just double checked)**

# Moonwave iOS - Build & Injection Tutorial

This tutorial guides you step-by-step through setting up your environment, building **Moonwave**, and injecting it into Fortnite on iOS.

---

## 1. Setting Up Your Mac

If you already have Xcode and Rust configured for iOS cross-compilation, skip to **Section 2**.

### Step 1: Install Xcode Tools
You need the Xcode Command Line Tools for iOS headers and linkers. So open terminal and copy paste:
```bash
xcode-select --install

```

### Step 2: Install Rust

If you don't have Rust installed, set up `rustup`. So open terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
source "$HOME/.cargo/env"

```

### Step 3: Add the iOS Architecture Target

By default, Rust only targets macOS. Add the 64-bit iOS target by running this command:

```bash
rustup target add aarch64-apple-ios

```

---

## 2. Building the Dylib

1. Open Terminal, clone the `iOS` branch, and move into the project folder:
```bash
git clone -b iOS https://github.com/PuppeDevelopment/Moonwave.git
cd Moonwave
```


*(Your terminal prompt will now be inside the root `Moonwave` project directory, which is usually saved in your user home folder `/Users/yourname/Moonwave`)*.

2. Run the release build targeting iOS ARM64:
```bash
cargo build --release --target aarch64-apple-ios

```


3. Locate your compiled `.dylib` file:
When Cargo finishes, it creates a `target` folder **inside** your current `Moonwave` directory.
Relative path from your current terminal position:
```text
target/aarch64-apple-ios/release/libmoonwave.dylib

```


Absolute path (if you need to copy-paste it into Sideloadly):
```bash
pwd # Run this in Terminal to see your full path, then add /target/aarch64-apple-ios/release/libmoonwave.dylib

```


*Tip: You can also run `open target/aarch64-apple-ios/release/` in Terminal to open the exact output folder in Finder.*


---

## 3. Injecting into Fortnite (Sideloadly)

### Requirements

* A **decrypted Fortnite `.ipa**` file.
* [Sideloadly](https://sideloadly.io/) installed on your Mac.

### Injection Steps

1. Connect your iPhone to your Mac via USB and open **Sideloadly**.
2. Drag and drop your Fortnite .ipa file into Sideloadly.
3. Click on **Advanced Options** (the gear icon).
4. Under **Inject dylib/extra files**, click **+** and select:
`target/aarch64-apple-ios/release/libmoonwave.dylib`
5. Enter your Apple ID at the top and click **Start**.
6. Once installed on your iPhone, go to **Settings > General > VPN & Device Management** and trust your Apple ID profile before opening Fortnite.


### HTTP / Local Server Fix (Allow `http://` Traffic)
By default, iOS blocks plain `http://` connections via App Transport Security (ATS). If your backend server uses `http://` instead of `https://` (no SSL certificate), you must patch the `.ipa` file manually:

1. Change your decrypted Fortnite `.ipa` file extension to `.zip` and extract it.
2. Open the extracted **`Payload`** folder.
3. Open the **`Info.plist`** file in a text editor (like VS Code, TextEdit, or Sublime).
4. Scroll to the bottom and paste this XML snippet directly **before** the final `</dict>` tag:
```xml
   <key>NSAppTransportSecurity</key>
   <dict>
       <key>NSAllowsArbitraryLoads</key>
       <true/>
   </dict>

```

5. Save and close `Info.plist`.
6. Zip the **`Payload`** folder back up and rename the file extension from `.zip` back to `.ipa`.


---

## Troubleshooting

* **App Fails to Connect to `http://` Server (`NSURLErrorDomain`)**  
  iOS App Transport Security (ATS) is blocking unencrypted traffic. Follow the **HTTP / Local Server Fix** steps in Section 3 to add `NSAllowsArbitraryLoads` to the IPA's `Info.plist`.
* **Error: `target 'aarch64-apple-ios' not found**`
Run `rustup target add aarch64-apple-ios` in your terminal.
* **Linker Errors (`ld: library not found`)**
Accept the Xcode license agreement in terminal:
```bash
sudo xcodebuild -license accept

```
