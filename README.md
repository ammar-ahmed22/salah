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
**Parameters:**
| Parameter             | Description                           | 
| :-------------------- | :------------------------------------ | 
| `--city <CITY>`       | The city to calculate timings for.    |
| `--country <COUNTRY>` | The country to calculate timings for. |

Uses the [Nominatim OpenStreetMaps API](https://google.ca) to get latitude/longitude values required for calculating prayer times.

##### `salah coord`
```
salah coord [OPTIONS] --lat <LAT> --lng <LNG> [TIMINGS]...
```

**Parameters:**
| Parameter     | Description                                   | 
| :------------ | :-------------------------------------------- | 
| `--lat <LAT>` | The latitude value to calculate timings for.  |
| `--lng <LNG>` | The longitude value to calculate timings for. |

Calculates timings directly without any need for external API calls.

#### `[OPTIONS]`
These options are the same for both `salah location` and `salah coord`
| Option                            | Description                                                                                                   | Format                  | Default           |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------ | :---------------------- | :---------------- |
| `-d` <br/>`--date <DATE>`         | Date to calculate the timings for. `today` for today's date according to the set timezone.                    | `YYYY-MM-DD` or `today` | `today`           |
| `-t` <br/>`--timezone <TIMEZONE>` | Timezone to output the timings for. All timezones under IATA are available.                                   | `Continent/Region`      | `America/Toronto` |
| `-a` <br/>`-all`                  | Calculates all the availabe prayer timings. Overrides any values in `[TIMINGS]...`                            | N/A                     | `false`           |
| `--hanafi`                        | Calculates Asr time using the Hanafi madhab                                                                   | N/A                     | `false`           |
| `--auth <AUTH>`                   | Calculation authority to use. Relevant for Fajr and Isha times. (use `salah authority`) for available values. | per `salah authority`   | `ISNA`            |
| `--format <FORMAT>`               | Format string to use for the timings output. Follows `strftime` from C language.                              | per `strftime`          | `%H:%M:%S`        |
| `-h, --help`                      | Print help                                                                                                    | N/A                     | `false`           |

#### `salah timings`
Lists all available values for `[TIMINGS]...`
```
Usage: salah <location | coords> [OPTIONS] [TIMINGS]...

The below can be passed to [TIMINGS]...

Timings:
  fajr      The dawn prayer time. Dependent on angle determined by authority (see salah authority)
  sunrise   Sunrise time. Fajr time ends at sunrise.
  dhuhr     The mid-day prayer time.
  asr       The evening prayer time. Dependent on school of thought (Hanafi vs Others).
  maghrib   The sunset prayer time.
  isha      The night prayer time. Dependent on angle determined by authority (see salah authority)
  midnight  The Islamic midnight time. Isha time ends at midnight.
```

#### `salah authority`
Lists all the available calculation authorities to use with `--auth <AUTH>`
```
Usage: --auth <AUTH>

Explanation:
Calculation authorities are used for the calculation of Fajr and Isha.
The time for Fajr is described as dawn; when there is fine white line at the horizon.
Isha time is described as when the night sky has lost all the light from the sunset.
As this is quite ambiguous, the scholars have differed upon the angle that the sun
makes when these two times occur. Each authority has slightly different angles for
Fajr and Isha. Makkah uses a time difference from Maghrib (sunset).

The below can be used with the --auth <AUTH> option when calculating timings.

Authorities:
  MWL       Fajr at 18 degrees, Isha at 17 degrees. - Muslim World League
  ISNA      Fajr at 15 degrees, Isha at 18 degrees. - Islamic Society of North America
  Egypt     Fajr at 19.5 degrees, Isha at 17.5 degrees - Egyptian General Authority of Survey
  Makkah    Fajr at 18.5 degrees, Isha 90 min after Maghrib. - Umm al-Qura University, Makkah
  Karachi   Fajr at 18 degrees, Isha at 18 degrees. - University of Islamic Sciences, Karachi
  Tehran    Fajr at 17.7 degrees, Isha at 14 degrees. - Institute of Geophysics, University of Tehran
  Jafari    Fajr at 16 degrees, Isha at 14 degrees. - Shia Ithna Ashari, Leva Research Institute, Qum
```

### 🚧 Examples
#### Getting Fajr time for Toronto, Canada, formatted
```
salah location --city Toronto --country Canada --date 2024-02-11 --format "%I:%M %p" fajr
```
##### Output:
```
fajr 06:03 AM
```

#### Getting fardh timings for Karachi, Pakistan, formatted
```
salah location --city Karachi --country Pakistan -t Asia/Karachi --format "%I:%M %p" --date 2024-02-11 fajr dhuhr asr maghrib isha
```

##### Output:
```
fajr 06:05 AM
dhuhr 12:46 PM
asr 03:59 PM
maghrib 06:23 PM
isha 07:27 PM
```

#### Getting Asr with Hanafi madhab for Toronto, Canada, formatted
```
salah location --city Toronto --country Canada --date 2024-02-11 --format "%I:%M %p" --hanafi asr
```

##### Output:
```
asr 03:57 PM
```
#### Getting all timings for Makkah, Saudi Arabia, formatted
```
salah location --city Makkah --country "Saudi Arabia" --date 2024-02-11 -t Asia/Riyadh --format "%I:%M %p" --auth Makkah --all
```

##### Output:
```
fajr 05:37 AM
sunrise 06:54 AM
dhuhr 12:35 PM
asr 03:51 PM
maghrib 06:16 PM
isha 07:46 PM
midnight 12:35 AM
```

### 🤝 Contributing
Contributions are welcome! I'm very new to Rust (this is my first project lol), so I'm sure there is a lot to improve on. Feel free to open a PR or create an issue!

### 📄 License
[MIT](./LICENSE)