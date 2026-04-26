# Refract — Ruby LSP for Zed

[Refract](https://github.com/hrtsx/refract) is a fast Ruby language server backed by SQLite. This extension integrates it into Zed.

## Features

- Type-aware completion — receiver inference, route helpers, i18n keys
- Hover — types, YARD docs, parameter descriptions, Rails DSL
- Go to definition / implementation / references — MRO-aware
- Rename — safe cross-file rename that respects inheritance and mixins
- Inlay hints — return types and block parameter types
- Diagnostics — Prism parse errors + RuboCop
- Rails — routes, associations, validations, callbacks, i18n
- HAML / ERB template support

## Install

Search for **Refract** in Zed's extension panel (`cmd+shift+x`).

The binary is downloaded automatically on first use. Alternatively, install it manually and set the path in settings:

```json
{
  "lsp": {
    "refract": {
      "binary": { "path": "/path/to/refract" }
    }
  }
}
```

## Configuration

Pass any [initializationOptions](https://github.com/hrtsx/refract#configuration) via Zed settings:

```json
{
  "lsp": {
    "refract": {
      "initialization_options": {
        "logLevel": 2,
        "disableRubocop": false,
        "maxWorkers": 0
      }
    }
  }
}
```
