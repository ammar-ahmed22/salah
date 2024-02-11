<div align="center">
  <img src="./images/salah_logo.png?raw=true" width="100" height="100" />
  <h2>salah</h2>
  <p>Rust-based command-line tool that efficiently calculates Islamic prayer times without external API calls, perfect for integrating accurate prayer schedules into servers and Islamic applications.</p>
</div>

### ✨ Features
- **Location Specific:** Calculate all the relevant Islamic prayer times based on specified locations.
- **Timezone Specific:** Output times based on defined timezones.
- **Independent on APIs:** Calculate times without any external API calls (need to know your latitude/longitude) (bringing major city support soon)
- **Configurable**: Highly configurable CLI to set any relevant calculation parameters (Calculation Authority, Madhab, etc.)

### 👨🏾‍💻 Installation
To install salah, you can use `cargo` or download the binary directly from the [latest release](https://github.com/ammar-ahmed22/salah/release)

#### Using `cargo`
```bash
cargo install salah
```

### 🤸🏾‍♂️ Usage
```bash
Usage: salah <COMMAND>

Commands:
  location   Use location (city/country) to get prayer times. WARNING: Uses external API call, network connection required
  coord      Use coordinates (latitude/longitude) to get prayer times
  timings    Lists all the available timings
  authority  Lists all the calculation authorities
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
#### Commands
##### `salah location`
```
salah location [OPTIONS] --city <CITY> --country <COUNTRY> [TIMINGS]...
```
**Options:**
| Option          | Description                        | Default | Required   |
| :-------------- | :--------------------------------- | :------ | :--------- |
| `--city <CITY>` | The city to calculate timings for. | None    | - [x]      |

##### `salah coord`

#### `salah timings`

#### `salah authority`

#### Examples
##### Getting all timings for Whitby, Canada

##### Getting fardh timings for Karachi, Pakistan

#### Getting Asr with Hanafi madhab for Whitby, Canada