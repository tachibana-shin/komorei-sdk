## Resource Files

Unique to Komorei source projects is the `res` directory, which contains additional files that are
packaged with the source when compiled. There is exactly one JSON configuration file,
`source.json`, which has a corresponding JSON schema that can be used for type hints and
autocompletion in IDEs.

To configure this for [Visual Studio Code](https://code.visualstudio.com/), you can set the
`json.schemas` key in `settings.json`:

```json
"json.schemas": [
	{
		"fileMatch": ["*/res/source.json"],
		"url": "https://raw.githubusercontent.com/komorei-sdk/komorei-sdk/refs/heads/main/crates/cli/src/supporting/schema/source.schema.json"
	}
]
```

Similarly for [Zed](https://zed.dev/) (recommended for Rust development), you can configure the
`json-language-server` parameters with the same array of schemas:

```json
"lsp": {
	"json-language-server": {
		"settings": {
			"json": {
				"schemas": [
					...
				]
			}
		}
	}
}
```

Note that this schema can also be checked against packaged sources by using `komorei`:

```sh
komorei verify package.krx
```

### source.json

The `source.json` file is the only required configuration file, comprised of an "info" object.
Every listing, filter, and setting is defined **dynamically in code** (via the `DynamicListings`,
`DynamicFilters`, and `DynamicSettings` traits) — there are no `listings`, `filters.json`, or
`settings.json` files. Here is an example of how the file should be structured:

```json
{
	"info": {
		"id": "en.example-source",
		"name": "Example Source",
		"version": 1,
		"url": "https://example.com",
		"contentRating": 0,
		"languages": ["en"]
	}
}
```

| Field                | Description                                                                                                                        |
|----------------------|------------------------------------------------------------------------------------------------------------------------------------|
| `info.id`            | A unique identifier for the source. Conventionally, this should be the language and source name, concatenated with a period.       |
| `info.name`          | The displayed name of the source.                                                                                                  |
| `info.altNames`      | An array of additional names that the source should be searchable by.                                                              |
| `info.version`       | The source's version number. It must be a positive integer and incremented with any notable changes.                               |
| `info.url`           | The source's main URL, which can be used for deep linking.                                                                         |
| `info.urls`          | An array of the source's URLs, If the source has multiple domains, this should be used instead of `info.url`.                      |
| `info.contentRating` | The NSFW level of the source. `0` for sources with no NSFW content at all, `1` for some NSFW, and `2` for majority NSFW sources.   |
| `info.languages`     | An array of ISO 639 language codes that the source supports.                                                                       |
| `info.minAppVersion` | Optional minimum Komorei app version supported by the source.                                                                      |
| `info.maxAppVersion` | Optional maximum Komorei app version supported by the source.                                                                      |

> **Note:** The source icon is provided as a separate `icon.png` resource (see below). There is no
> `icon` field in `source.json`.

### icon.png

Komorei expects a PNG image file to display as the source icon. The image should be square, and
128x128 with no transparency — the `komorei verify` command enforces both requirements.