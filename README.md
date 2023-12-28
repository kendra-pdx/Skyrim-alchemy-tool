# Skyrim Alchemy Tool

## CLI Installation
`cargo install --path skyrim_alchemy_cli`

## CLI Usage
Run `skyrim_alchemy_cli` listing your entire ingredient inventory on STDIN.

* Take a photo of your ingredients with your iPhone
* Select all of the text and copy it to your clipboard (don't worry about duplicates)
* Either paste it to a file and pipe it into the cli, or paste it into your terminal after running.

## Web Build

- `wasm-pack build --target web skyrim_alchemy_wasm`
- `cd skyrim_alchemy_web`
    - `npm run build`

### Prerequisites
- `cargo install wasm-pack`

## Data Sources
See `skyrim_alchemy_db/fetch_data.sh` and run it.
Data credit to http://alchemy-eight.vercel.app ([Source](https://github.com/Finbel/alchemy))
