# Filter Management Implementation Plan

## Architecture Decision
Use per-component signals (`use_signal`) in both `CustomFilter` and `Filters`, with an `use_effect` in `Filters` to re-sync from LocalStorage when the signal changes.

---

## [ ] Step 1: Add filter management methods to `Filter` in `src/model.rs`

Add these methods to the `impl Filter` block:

- `new_custom(factor: u64, fstop: f64, name: String) -> Filter` — create a new Filter with auto-incremented ID
- `next_id(&self) -> usize` — compute next unique ID (max existing + 1, or 0 if empty)
- `reset_to_defaults() -> Vec<Filter>` — return `default_filter_list()`
- Update `remove_filter` to be a free function or add to impl

## [ ] Step 2: Update `CustomFilter` component in `src/settings.rs`

Rewrite `CustomFilter` to:
- Use `use_signal(Filter::filter_list)` for writable state
- Add handlers: `add_filter()`, `remove_filter(id)`, `reset_to_defaults()`
- Persist all changes to LocalStorage via `Filter::store_filter_list()`
- Wire delete buttons to `remove_filter` callback
- Render "Add Filter" and "Reset to Defaults" buttons below the list

## [ ] Step 3: Update `Filters` component in `src/calculator.rs`

Add `use_effect` to re-sync from LocalStorage when the signal changes:
```rust
use_effect(move || {
    let mut sig = filters.write();
    *sig = Filter::filter_list();
});
```

## [ ] Step 4: Add i18n keys

### `locales/en.yml`
```yaml
filter.add: "+ Add Filter"
filter.reset: "Reset to Defaults"
```

### `locales/de-DE.yml`
```yaml
filter.add: "+ Filter hinzufügen"
filter.reset: "Auf Standard zurücksetzen"
```

## [ ] Step 5: Fix test in `src/model.rs`

In `test_filter_default_list`, fix ND1000 id from `3` to `2` (sequential):
```rust
assert_eq!(filters[2].id, 2);  // was 3
```

---

## Files Modified
1. `src/model.rs` — new methods + test fix
2. `src/settings.rs` — `CustomFilter` rewrite
3. `src/calculator.rs` — `Filters` sync effect
4. `locales/en.yml` — 2 new keys
5. `locales/de-DE.yml` — 2 new keys
