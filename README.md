# rustsy: An Educational Synth in Rust
Copyright (c) 2022 Bart Massey

*This is a work-in-progress. As of now, it does nothing
interesting. Please be patient for a bit while I get it to
where it needs to be.*

This project is a simple "educational" sound and music
synthesizer. It is intended not so much as a finished
instrument as a demo of basic synth functionality of several
kinds.

Much of this project will be ported from my
[fm](http://github.com/pdx-cs-sound/fm) educational synth
written in Python. The sampler, the audio and MIDI
interfaces, and some of the structure are borrowed from my
[synthkit](http://github.com/pdx-cs-sound/synthkit) sampling
synth written in Rust. This code borrows its Git history
from that project: see the branches in this repo labeled
`synthkit-` for details.

## Acknowledgments

Thanks to Ron Nicholson for the BASIC code for filtered
linear interpolation which was adapted to become the heart
of the resampler. Thanks to the PDX Rust group for valuable
code feedback and suggestions.

## License

This program is licensed under the "MIT License".  Please
see the file LICENSE in the source distribution of this
software for license terms.
