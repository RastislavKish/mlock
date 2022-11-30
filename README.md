# mlock

Since certain version, Ubuntu Mate introduces a bug, where closing particular types of apps (like Chromium apps, but VLC as well, for example) causes the keyboard navigation to freeze until mouse is moved in any direction.

While I'm not sure what exactly is responsible for this issue and how (it's not mate-desktop/marco#703), my testing revealed, that it only happens when the mouse cursor is in a certain location on the screen, namely a particular shape in the center. Outside this region, everything works as expected, without any problems.

So, while it would be awesome if this bug got properly repaired, as a temporal workaround, I created Mlock.

## How does it work

The idea is very simple. Mlock, standing for Mouse location keeper, puts the mouse cursor to a specified location (100;100 by default) and prevents it from running further than a specified amount of pixels horizontally or vertically.

This way, the mouse shouldn't be residing in the problematic region, and thus in turn, the desktop environment should work as expected.

One problem of this approach is, that you may want to actually use the mouse in some instances, such as clicking on objects or lending your device to a sighted person. Since Mlock is still rather experimental, it will give you 10-20 seconds after the mouse leaves the allowed window, this should be enough to perform simple UI control operations.

A more sophisticated solution may be implemented in the future.

Don't forget to turn the program off before leaving your computer to a sighted person, otherwise their experience may be quite confusing.

## Build

## Dependencies

* [Rust programming language](https://www.rust-lang.org/tools/install)
* libxdo-dev, for handling mouse operations

After installing the Rust programming language, Install the libxdo library on Ubuntu as follows:

```
sudo apt install libxdo-dev
```

### Compilation and usage

```
git clone https://github.com/RastislavKish/mlock
cd mlock
cargo build --release -q
# Optional, if there are no errors, move to your bin folder
sudo mv target/release/mlock /usr/local/bin
```

See mlock --help for usage instructions. It's recommended to setup Mlock to start with your system, so you get the most seamless experience using your DE.

## License

Copyright (C) 2022 Rastislav Kish

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, version 3.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.

