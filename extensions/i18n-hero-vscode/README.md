# i18n-hero-vscode

VSCode extension for [i18n-hero](https://github.com/hazyio/i18n-hero) — autocomplete, go-to-definition, and refactoring for translation keys across all string literals.

## Features

Unlike other i18n extensions that only activate inside known translation function calls like `t("key")`, i18n-hero completes on **all string literals** — `""`, `''`, and ` `` `. This means it works with custom widget abstractions that accept plain strings and call the translation function internally.

- **Autocomplete** — suggests translation keys as you type inside any string literal
- **Go to definition** — jump from a key in your code to its entry in the translation file
- **Hover** — see the translated value inline without leaving your editor
- **Rename** — rename a key and propagate the change across all locale files
- **Diagnostics** — warnings for keys used in code but missing from translation files

Supports **JSON**, **YAML**, and **TOML** translation file formats.

## Requirements

The extension requires the `i18n-hero` CLI binary. On first activation, the extension will offer to download it automatically for your platform. Alternatively, install it manually:

```sh
cargo install i18n-hero
```

## Extension Settings

| Setting | Default | Description |
|---|---|---|
| `i18n-hero.enabled` | `true` | Enable or disable the extension |
| `i18n-hero.translationFiles` | `""` | Glob pattern for translation files. Auto-detected if empty |
| `i18n-hero.defaultLocale` | `"en"` | Locale to use as the source of truth for completions |
| `i18n-hero.keySeparator` | `"."` | Separator used in nested keys |
| `i18n-hero.binaryPath` | `""` | Path to the `i18n-hero` binary. Uses PATH if empty |

## Known Issues

- Template literals with expressions (`` `Hello ${name}` ``) do not trigger completions — only plain template literals do.
- TOML support requires `i18n-hero` v0.2.0 or later.

## Release Notes

### 0.1.0

Initial release. Autocomplete, go-to-definition, hover, and diagnostics for JSON and YAML translation files.
