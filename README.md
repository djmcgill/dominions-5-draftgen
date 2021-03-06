# Dominions 5 Draft Gen

This program is intended to help you generate mods for use in drafts. Currently only supports "simple" drafts i.e. you draft base nation (including national spells, cap-only recruits, etc), non-cap troops, and non-cap mages.
You will probably need to then modify the file further.

It is based on the data from the dominions 5 mod inspector: https://larzm42.github.io/dom5inspector/

Currently it does not support non-fort recruits, so for things like forest/uw/coast recruits you'll need to tweak the result.

Also apparently it's messed up for TC? See:
> for certain nations (and I don't know why it is these certain nations, but it is), their recruitment is more difficult to modify than normal; TC is one of these, for example, sometimes you need to use #landcom to denote commanders recruitable in forts where addreccom actually works as foreign recruit and addforeignreccom adds things as foreign units.
>
> -- jBrereton

## Usage
### From source
```bash
# REQUIRES: Git & Rust

git clone https://github.com/djmcgill/dominions-5-draftgen.git
cd dominions-5-draftgen
cargo run -- <base_nation> <commander_nation> <troop_nation>

# e.g. cargo run -- 83 47 47
```
Then save the output into a file with a .dm extension and drop it into your dominions 5 mods folder (for windows users that's `%APPDATA%/Dominions5/mods`).

#### Example Output:
```
#modname "dumb draft mod"
#selectnation 83
#clearrec

#addreccom 56
#addreccom 54
#addreccom 60
#addreccom 658
#addreccom 151
#addreccom 152


#addrecunit 1
#addrecunit 2127
#addrecunit 61
#addrecunit 62
#addrecunit 53
#addrecunit 59
#addrecunit 63
#addrecunit 2129
#addrecunit 57


#end
#end
```

## TODO
- More types of recruitment
- Binary release
- More than one nation at a time
- More types of draft

## LICENSE
GPLv3
