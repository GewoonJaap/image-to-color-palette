![afbeelding](https://user-images.githubusercontent.com/33700526/207815865-9b471652-5723-4d35-8847-dce0fb9701eb.png)

# Image to Color Palette Cloudflare Worker

Extract a color palette from an image URL using Cloudflare Workers.

# Installation

## Windows Specific
- Install [Strawberry Perl](https://strawberryperl.com/)

## All OS
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Cloudflare Wrangler](https://developers.cloudflare.com/workers/cli-wrangler/install-update)
- `wrangler login`
- Create a Cloudflare worker with name: `svg-to-png`;
- `wrangler dev` to local test
- `wrangler publish` to publish to Cloudflare

# Usage

`https://image-to-color-palette.mrproper.dev/{IMAGE URL}`

[Demo]: https://image-to-color-palette.mrproper.dev/https://user-images.githubusercontent.com/33700526/207815865-9b471652-5723-4d35-8847-dce0fb9701eb.png

## Return value
    
```json
{
    "hex": ["#fbbf5f", "#1d1d1d", "#ba560a", "#8c7444", "#715c38", "#54442c", "#544434", "#463a24", "#342c24"],
    "rgb": ["251,191,95", "29,29,29", "186,86,10", "140,116,68", "113,92,56", "84,68,44", "84,68,52", "70,58,36", "52,44,36"]
}
```
Each entry in the `rgb` object is a RGB color value in the format `R,G,B`.
