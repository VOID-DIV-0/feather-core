# nekonomicon Syntax Extension for Zed

Syntax highlighting and language support for nekonomicon (`.spell`) scripting language in [Zed](https://zed.dev/).

## Features

- Syntax highlighting for `.spell` files
- Comment support (line comments with `~`)
- Recognition of:
  - Keywords: `if`, `else`, `end`, `repeat`, `while`, `success`, `failure`
  - Clauses: `safe`, `async`, `sensitive`, `<!>sensitive`, `elevated`
  - Variables: `@variable` (scalar), `::container` (container), `@!sealed` (sealed)
  - Projections: `::container:field`
  - Signatures: `trace`, `elapsed`, `timeout`, `silent`, `on`
  - String interpolation: `@{var}`, `::{container:field}`

## Installation

### Option 1: Install from Zed Extensions (Coming Soon)

Once published to the Zed extension registry:

1. Open Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
3. Type "zed: extensions"
4. Search for "nekonomicon"
5. Click Install

### Option 2: Manual Installation (Local Development)

1. Clone this repository or copy the `nekonomicon-core-zed-extension` folder
2. Create the Zed extensions directory if it doesn't exist:
   ```bash
   mkdir -p ~/.config/zed/extensions
   ```
3. Link or copy the extension to Zed's extensions directory:
   ```bash
   ln -s /path/to/nekonomicon-core/packages/nekonomicon-core-zed-extension ~/.config/zed/extensions/nekonomicon
   ```
4. Restart Zed

## Usage

Open any `.spell` file in Zed and the syntax highlighting will be automatically applied.

### Example nekonomicon Code

```spell
~ This is a comment
'Hello, World!' into @greeting.

vault unlock 'api_key' into @key.
sensitive http get 'https://api.example.com' with header 'Authorization' 'Bearer @{key}' into @response.

if @response
  say 'Success!'.
else
  say 'Failed.'.
end

success.
```

## Language Features

- **Comments**: Lines starting with `~`
- **Variables**:
  - Scalar: `@variable`
  - Sealed: `@!variable`
  - Container: `::container`
  - Sealed container: `!::container`
  - Projection: `::container:field`
- **Clauses**: `safe`, `async`, `sensitive`, `(!)sensitive`, `elevated`
- **Control flow**: `if`, `else`, `end`, `repeat`, `while`
- **Results**: `success`, `failure`

## About nekonomicon

nekonomicon is a new language for scripting and automation that prioritizes readability, safety, and cross-platform compatibility. Learn more at [github.com/VOID-DIV-0/nekonomicon-core](https://github.com/VOID-DIV-0/nekonomicon-core).

## License

See the main repository for license information.

## Contributing

Contributions are welcome! Please submit issues and pull requests to the main repository.
