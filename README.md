# Constructor Asset Extraction
Constructor was a classic game released in 1997 by Acclaim. This repo contains scripts which extract the assets from their ancient formats to more usable formats. The code is extremely hacky, ugly and shit, but it does work reliably to get the assets out, which is all that matters for such an obscure purpose :stuck_out_tongue:

### Requirements
* A recent version of [GoLang](//www.golang.org)
* Ruby and [Bundler](//bundler.io)

### Why?
Other classic games from that era have enhanced open source clones e.g.

* [Theme Hospital as CorsixTH](//github.com/CorsixTH)
* [Transport Tycoon as OpenTTD](//www.openttd.org/en/)

I was unable to find a similar clone for constructor, which is sad because it was a great game in its time. Writing a clone of any game (even a very old one) is a huge undertaking, but once it's possible to extract the assets, the barrier for experimenting or making a prototype becomes much lower.

### How?
The credit for working out the more complicated part of decrypting the files and determining their format belongs to @shlainn who [described the file formats in detail](https://github.com/shlainn/game-file-formats).

This repo contains:
* One script in Golang (extract.go) to extract the DATA.FIL archives, which wrap all the essential assets for the game.
* One script in Ruby (sprite.rb) to extract the sprites and the isometric tiles, then order the tiles into a large jpg which shows the whole map

To use the script:
* Obtain a copy of constructor from your favourite abandonware site (or the original game CD if any of them still exist)
* Copy the CONST/ folder to the same directory as the script

##### Why two languages?
I implemented the sprite extraction in Ruby first after using the FILedit tool from @shlainn to extract the assets files and everything worked ok. The decryption process for the FIL files was very difficult to manage in Ruby because of how it interprets binary integers, so I wrote the FIL extraction script in Golang.

I also attempted to clone the logic for extracting sprites in Golang, but found the language unsufferable, at least for that purpose. I left the code in the extract.go file for future reference but commented it out.

### What Next?
The script is currently very dumb and just spits out a load of extracted files or .png versions of the sprites. Ideally it would:
* Produce sprite sheets that could be dropped straight into [Phaser.js](http://phaser.io/) or other game libraries
* Encode the .WAV files to .ogg or something more web-friendly
* Encode the videos to Theora or just extract the frames because they're mostly very short?

### Known Issues
* The SFX/DATA.FIL file has formatting which causes the script to fail and miscalculate the offsets about halfway through the file. Therefore only about half the .wav files are extracted. This is likely due to an entry with 0 length, which seems to be a common feature (or problem) with the formats the game uses.
* Probably lots more...


