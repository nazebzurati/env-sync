# env-sync

A simple dotenv file syncer that keeps your messy `.env` in sync with your template, placing values in the correct order.

If you encounter any issues, please report them promptly [here](https://github.com/nazebzurati/env-sync/issues). Additionally, feel free to request any necessary features [here](https://github.com/nazebzurati/env-sync/issues).

## Installation

TBA

## Try out!

```
env-sync -i sample/.env -t sample/.env.template -o sample/.env.output
```

## Use case

- Clean up a messy `.env` without hunting variables by hand: sync it with `.env.example` to drop unused keys and regenerate an up-to-date template.
- If your `.env` got reordered (sorted, shuffled, comments lost), sync it to restore the same order/structure as `.env.example` so it’s easy to compare and review.
