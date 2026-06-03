# Translation guide

Clippy Land uses Fluent translation files stored in `i18n/<locale>/cosmic_applet_clippy_land.ftl`.

Fallback language configuration lives in `i18n.toml` and currently uses `en`.

## Current translations and contributors

- English — [@k33wee](https://github.com/k33wee)
- Italian — [@k33wee](https://github.com/k33wee)
- Portuguese — [@GuilhermeTerriaga](https://github.com/GuilhermeTerriaga)
- Czech — [@lorduskordus](https://github.com/lorduskordus)
- Ukrainian — [@Dymkom](https://github.com/Dymkom)
- Swedish — [@bittin](https://github.com/bittin)
- French — [@Thovi98](https://github.com/Thovi98)
- Polish — [@VandaLHJ](https://github.com/VandaLHJ)

## Adding a new translation

1. Create a new directory under `i18n/` using the locale code, for example:

```text
i18n/es/
```

2. Copy the English base file:

```text
i18n/en/cosmic_applet_clippy_land.ftl
```

3. Translate every message into the target language.

## Updating an existing translation

When new strings are added, compare your locale file with the English file and make sure all keys are present.

Current message file format example:

```text
empty = Clipboard is empty
remove = Remove
pin = Pin
unpin = Unpin
delete-all = Clear History
search-placeholder = Search in clipboard history
no-results = No results found
```

## Translation rules

- Keep every message key exactly the same.
- Only translate the message values.
- Preserve placeholders and Fluent syntax if new strings introduce them.
- Keep the file name exactly `cosmic_applet_clippy_land.ftl`.
- Use UTF-8 text.
- Try to keep labels short enough for panel popup controls.

## Translation PR checklist

Please include in your PR:

- the locale code you added or updated
- whether it is a new translation or an update
- your preferred contributor tag for credits in this file / README-style docs
- confirmation that you checked your file against `i18n/en/cosmic_applet_clippy_land.ftl`

Suggested PR description:

```md
## Translation

- Locale: xx
- Type: new translation / update

## Notes

- Added or updated all keys from `i18n/en/cosmic_applet_clippy_land.ftl`
- Contributor tag for credits: @your-handle
```

## Translation PR expectations

- One locale per PR is preferred unless you are updating a shared string set across multiple languages.
- Keep translation PRs focused; avoid mixing code changes unless they are necessary for new strings.
- If you add a brand-new locale, also add yourself to the contributor list in this file.

## Related files

- `i18n.toml`
- `i18n/en/cosmic_applet_clippy_land.ftl`
- `src/i18n.rs`
