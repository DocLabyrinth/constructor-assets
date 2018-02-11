# Constructor Asset Extraction
Constructor was a classic game released in 1997 by Acclaim. This repo contains scripts which extract the assets from their ancient formats to more usable formats. The code is extremely hacky, ugly and shit, but it does work reliably to get the assets out, which is all that matters for such an obscure purpose :stuck_out_tongue:

### Requirements
* [Rust and Cargo](//www.rustup.rs/)
* Ruby and [Bundler](//bundler.io)
* (optional) A recent version of [GoLang](//www.golang.org)

### Why?
Other classic games from that era have enhanced open source clones e.g.

* [Theme Hospital as CorsixTH](//github.com/CorsixTH)
* [Transport Tycoon as OpenTTD](//www.openttd.org/en/)

I was unable to find a similar clone for constructor, which is sad because it was a great game in its time. Writing a clone of any game (even a very old one) is a huge undertaking, but once it's possible to extract the assets, the barrier for experimenting or making a prototype becomes much lower.

### How?
The credit for working out the more complicated part of decrypting the files and determining their format belongs to @shlainn who [described the file formats in detail](https://github.com/shlainn/game-file-formats/wiki).

This repo contains:
* A rust project which builds to a binary which can inspect the contents of DATA.FIL files and extract their contents to a given target directory
* One script in Ruby (sprite.rb) to extract the sprites and the isometric tiles, then order the tiles into a large jpg which shows the whole map
* One script in Golang (extract.go) to extract the DATA.FIL archives which is now deprecated but left in the repo for future reference

To use the script:
* Obtain a copy of constructor from your favourite abandonware site (or the original game CD if any of them still exist)
* Copy the CONST/ folder to the same directory as the script

To use the rust binary:

```bash
# print out the filenames and sizes of all the files contained in the .FIL
cargo run -- fil inspect -f ../CONST/DATA/DATA.FIL

# extract the files from the .FIL into a directory
cargo run -- fil extract -f ../CONST/DATA/DATA.FIL --output-dir ../output
```

##### Why three languages?
I implemented the sprite extraction in Ruby first after using the FILedit tool from @shlainn to extract the assets files and everything worked ok. The decryption process for the FIL files was very difficult to manage in Ruby because of how it interprets binary integers, so I wrote the FIL extraction script in Golang.

I also attempted to clone the logic for extracting sprites in Golang, but found the language unsufferable, at least for that purpose. I left the code in the extract.go file for future reference but commented it out.

As an exercise to get to grips with Rust, I used it to reimplement the functionality from extract.go in a way that should hopefully be easier to pick up and use if you already have the rust development tools installed. The project is located in the [/rust folder](/rust)

### What Next?
The script is currently very dumb and just spits out a load of extracted files or .png versions of the sprites. Ideally it would:
* Produce sprite sheets that could be dropped straight into [Phaser.js](http://phaser.io/) or other game libraries
* Encode the .WAV files to .ogg or something more web-friendly
* Encode the videos to Theora or just extract the frames because they're mostly very short?

### Known Issues
* For unknown reasons (possibly an overflow of the integer type or something similar) the extract.go script could only extract about half of the files in SFX/DATA.FIL It seems like the rust binary is able to extract all of the files successfully, but there may be some problems.
* Several files seemed to have 0-length entries which might confuse the script, but the main files containing the map data, the sprites, the sound-effects and videos seem to extract successfully.
* Probably lots more...
