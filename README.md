# choice_exchange Substreams modules

This package was initialized via `substreams init`, using the `injective-events` template.

## Usage

```bash
substreams build
substreams auth
substreams gui       			  # Get streaming!
```

Optionally, you can publish your Substreams to the [Substreams Registry](https://substreams.dev).

```bash
substreams registry login         # Login to substreams.dev
substreams registry publish       # Publish your Substreams to substreams.dev
```

## Modules

### `map_events`

This module uses the [Injective Foundational Modules](https://github.com/streamingfast/substreams-foundational-modules/)
and applies filters on the chosen events and attributes.
