<h1 align="center">zj-hooks 🔀🧠</h1>

<p align="center">
  Hook commands and scripts to zellij events.
  <br><br>
  <a href="https://github.com/dj95/zj-hooks/actions/workflows/lint.yml">
    <img alt="clippy check" src="https://github.com/dj95/zj-hooks/actions/workflows/lint.yml/badge.svg" />
  </a>
  <a href="https://github.com/dj95/zj-hooks/releases">
    <img alt="latest version" src="https://img.shields.io/github/v/tag/dj95/zj-hooks.svg?sort=semver" />
  </a>
  <a href="https://github.com/dj95/zj-hooks/wiki">
    <img alt="GitHub Wiki" src="https://img.shields.io/badge/documentation-wiki-wiki?logo=github">
  </a>
</p>

## 🚀 Usage

When zj-hooks is installed, simply add it with its configuration to the layout you like.

Hooks in the configuration must be named alphanumeric. Configuration of a hook consists of the following parts:

- **hook_NAME_event** The type of event that should trigger the hook. Please refer to current supported events.
- **hook_NAME_command** The command that should be run on the event type. It can contain placeholders, that are replaced by certain event details.

```javascript
plugin location="file:path/to/zj-hooks.wasm" {
    hook_test_event     "session"
    hook_test_command   "bash -c 'echo {{session_name}} >> test.log'"
}
```

### Supported Events

| Event Type | Supported placeholders |
|------------|------------------------|
| `session`  | `{{session_name}}` |
| `tab` | `{{active_tab_position}}` `{{active_tab_name}}` |
| `pane` | |
| `mode` | `{{mode}}` |

## 📦 Installation

Download the latest binary in the GitHub releases. Place it somewhere, zellij is able to access it. Then the
plugin can be included by referencing it either via [plugin aliases](https://zellij.dev/documentation/plugin-aliases) or directly in a layout file.

You could also refer to the plugin guide from zellij, after downloading the binary: [https://zellij.dev/documentation/plugin-loading](https://zellij.dev/documentation/plugin-loading)

```javascript
plugins {
  zj-hooks location="file:/abolute/path/to/zj-hooks.wasm"
}
```

## ❄️ Installation with nix flake

Add this repository to your inputs and then with the following overlay to your packages.
Then you are able to install and refer to it with `pkgs.zj-hooks`. When templating the
config file, you can use `${pkgs.zj-hooks}/bin/zj-hooks.wasm` as the path.

```nix
inputs = {
# ...

zj-hooks = {
  url = "github:dj95/zj-hooks";
};
};


# define the outputs of this flake - especially the home configurations
outputs = { self, nixpkgs, zj-hooks, ... }@inputs:
let
inherit (inputs.nixpkgs.lib) attrValues;

overlays = with inputs; [
  # ...
  (final: prev: {
    zj-hooks = zj-hooks.packages.${prev.system}.default;
  })
];
```


## 🤝 Contributing

If you are missing features or find some annoying bugs please feel free to submit an issue or a bugfix within a pull request :)

## 📝 License

© 2024 Daniel Jankowski

This project is licensed under the MIT license.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
