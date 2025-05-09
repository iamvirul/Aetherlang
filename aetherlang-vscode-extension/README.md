# Aetherlang VS Code Extension

Provides Visual Studio Code support for the Aetherlang programming language.

## Features

*   **Syntax Highlighting:** Basic syntax highlighting for Aetherlang (`.ath`) files.
*   **Language Configuration:** 
    *   Block comments: `{- ... -}`
    *   (Assumed) Single-line comments: `// ...` (Please update if this is incorrect for Aetherlang)
    *   Bracket matching and autoclosing for `()`, `{}`, `[]`.
*   **Extension Icon:** The Aetherlang logo is used as the icon for this extension in the VS Code Marketplace/Extensions view.
*   **File Icon (Optional Theme):** This extension includes a dedicated file icon theme named "Aetherlang Icons" that provides an icon for `.ath` files.
    *   ![Aetherlang File Icon](https://firebasestorage.googleapis.com/v0/b/android-firebase-b8116.firebasestorage.app/o/aether-extention.png?alt=media&token=fe691c68-47c5-4c4d-9e35-24f7f6d2b16c)

## Aetherlang File Icon

This extension provides a specific icon for Aetherlang (`.ath`) files through its own file icon theme called "Aetherlang Icons".

**Important Note:** To see the Aetherlang file icon, you will need to activate the "Aetherlang Icons" theme. You can do this by:

1.  Going to `File > Preferences > Theme > File Icon Theme` (or `Code > Settings > Theme > File Icon Theme` on macOS).
2.  Selecting "Aetherlang Icons" from the list.

**Caveat:** The "Aetherlang Icons" theme currently *only* provides an icon for `.ath` files. If you select this theme, icons for other file types (e.g., `.js`, `.py`, `.txt`) provided by your previously active icon theme will likely disappear or revert to basic defaults. 

For seamless integration with popular icon themes (like Material Icon Theme, Seti, etc.), we plan to contribute the Aetherlang icon to those themes in the future.

## Installation

1.  Download the `.vsix` file from the [releases page](https://github.com/iamvirul/Aetherlang/releases/tag/v1.1.0).
2.  In VS Code, open the Command Palette (`Ctrl+Shift+P` or `Cmd+Shift+P`).
3.  Run the command "Extensions: Install from VSIX...".
4.  Select the downloaded `.vsix` file.
5.  Reload VS Code if prompted.

## About Aetherlang

Aether is a modern, cloud-native programming language designed for building scalable, distributed systems with elegant syntax and robust safety features.

## License

This extension is licensed under the [MIT License](LICENSE) (refer to the LICENSE file in the main Aetherlang project).
