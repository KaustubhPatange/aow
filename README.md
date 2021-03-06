# Adb Over Wifi (aow)

[ /**a**-_o_/ ]

<a href="https://crates.io/crates/aow"><img src="https://img.shields.io/crates/v/aow.svg" alt="cargo"></a>
![CI](https://github.com/KaustubhPatange/aow/workflows/CI/badge.svg)

A command line tool written in **Rust** for adb to connect device to your machine over wifi.

On top of that it can do a lot of things (check full menu [here](https://github.com/KaustubhPatange/aow/wiki/Command-line-options)) for eg taking a screenshot, recording screen, testing deeplinks etc.

The program is made to handle multiple connected devices & also provide some _Hints_ if any error occurs with `adb`. It can smartly notify you about the new version available.

The usage of the program (after [installation](#Installation)) is pretty simple. Just connect a device > open a terminal & type `aow`. For more options read [here](https://github.com/KaustubhPatange/aow/wiki/Command-line-options).

![](art/demo.gif)

## Installation

The program is available for all major platforms _**Mac, Linux & Windows**_ however the installation procedure might differ.

- [Universal](https://github.com/KaustubhPatange/aow/wiki/Installation#universal)
- [Windows](https://github.com/KaustubhPatange/aow/wiki/Installation/#os)
- [Linux](https://github.com/KaustubhPatange/aow/wiki/Installation/#linux)
- [Mac](https://github.com/KaustubhPatange/aow/wiki/Installation/#mac)

Some information you might be interested in reading,

- [How it works?](https://github.com/KaustubhPatange/aow/wiki/FAQs#how-it-works)
- [Why does this tool exists?](https://github.com/KaustubhPatange/aow/wiki/FAQs#why-it-exists)

## Contribute

Contributions are very welcome! See [CONTRIBUTING](CONTRIBUTING.md) for more info.

## License

- [The Apache License Version 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt)

```
Copyright 2020 Kaustubh Patange

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
