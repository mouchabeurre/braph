# Braph
### Bar Graphs for your Shell

Inspired from Jason A. Donenfeld's [spark](https://git.zx2c4.com/spark/).

## Installation

The program is built using `cargo` (*which comes with [rustup](https://www.rust-lang.org/tools/install)*).

To install, run `make` and `sudo make install` (`DESTDIR` defaults to `/usr/local`).

## Usage

`braph` outputs a unicode *bar graph* based off the number list it recieves from **parameters** or **standard input** (parameters precede stdin).
The range of numbers is mapped to fit in 8 unicode bars: `▁▂▃▄▅▆▇█`.
Numbers can be floats.

### Via parameters

Each value must be passed as a distinct parameter (space-separated):
```
$ braph 41.2 25.3 10.8 120.7 87.4 65.1 23.0
▃▂▁█▆▄▂
```

### Via standard input

Every character that is not in `[0-9\-\.]` is considered a separator, such as both following streams yield the same output:
```
$ echo "1 2 3 4 5 6" | braph
▁▂▄▅▇█
$ echo "1:2_3ù4,5/6" | braph
▁▂▄▅▇█
```

## Examples

Characters per line in main.rs and lib.rs:
```
$ echo "$(awk '{print length($0)}' src/main.rs) $(awk '{print length($0)}' src/lib.rs)" | grep -Ev '^0$' | braph
▂▂▂▆▅▅▃▁▂▄▃▅▃▄▂▂▅▃▃▂▄▄▂▂▄▃▃▇▅▇▇▄▃▆▃▃▆▃▄▆▃▃▆▅▂▂▂▅▆▂▆▃▆▃▅▂▃▂▄▃▁▁▆██▄▃▄▇▇▅▇▂▂▁▅█▇▁
```

Bitcoin value history (31 last days):
```
$ curl -s https://api.coindesk.com/v1/bpi/historical/close.json \
| jq ".bpi | to_entries | .[] | .value" | braph
█▇▇▇▇▆▆▆▅▅▅▃▂▂▁▂▂▃▃▄▃▃▂▂▂▃▃▃▃▂▂
```
