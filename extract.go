package main

import (
  "bufio"
  "encoding/binary"
  "fmt"
  "os"
  "strings"
  "io/ioutil"
  "path/filepath"
  "image"
  "image/png"
  "image/color"
)

type FilIndexItem struct {
  filename string
  offset uint32
  length uint32
  start uint32
  end uint32
}

type LevelIndexItem struct {
  id string
  offset uint32
  length uint32
}

type ColorMapEntry struct {
  transparent bool
  r uint8
  g uint8
  b uint8
}

type SpriteItem struct {
  offset uint32
  length uint32
}

func GetIffIndex(sub_dir string, fil_file *os.File) ([]FilIndexItem) {
  // get the encrypted value for the number of entries in the file
  var (
    num_entries uint32
    err error
  )

  buffer := bufio.NewReader(fil_file)
  enc_entries := make([]byte, 4)

  file_info, err := fil_file.Stat()
  if err != nil {
    fmt.Println("Failed to stat DATA.FIL file: ", err)
    panic(err)
  }

  _, err = buffer.Read(enc_entries)
  if err != nil {
    fmt.Println("Failed to read the number of entries", err)
    panic(err)
  }

  // decrypt the value to get the number of entries
  num_entries = binary.LittleEndian.Uint32(enc_entries) ^ 0x3BD7A59A
  fmt.Println("entries: ", num_entries)

  index_buf := make([]byte, 17 * num_entries)
  index := make([]FilIndexItem, num_entries)

  _, err = buffer.Read(index_buf)
  if err != nil {
    fmt.Println("Failed to read the index data", err)
    panic(err)
  }

  // decrypt the index
  var current_byte byte

  for bit_i := 0; bit_i < len(index_buf); bit_i++ {
    current_byte = index_buf[bit_i]

    current_byte -= 39
    current_byte ^= 0xA5
    current_byte -= (byte)(27 + bit_i)

    index_buf[bit_i] = current_byte
  }

  err = ioutil.WriteFile( strings.Join([]string{sub_dir, "idx"}, "."), index_buf, 0644)
  if err != nil {
    panic(err)
  }

  // get the filenames and offsets in the file
  for index_i := uint32(0); index_i < num_entries; index_i++ {
    filename_start := 17 * index_i
    offset_start := filename_start + 13

    index[index_i].filename = string(index_buf[filename_start:offset_start])
    index[index_i].offset = binary.LittleEndian.Uint32(index_buf[offset_start:])
    index[index_i].start = filename_start
    index[index_i].end = offset_start + 4
  }

  // infer the lengths of each entry
  for index_i := uint32(0); index_i < num_entries; index_i++ {
    if index_i + 1 == num_entries {
      index[index_i].length = uint32(file_info.Size()) - index[index_i].offset
    } else {
      index[index_i].length = index[index_i + 1].offset - index[index_i].offset
    }
  }

  return index
}

func ExtractFile(sub_dir string) {
  var (
    file *os.File
    err error
    file_path string
  )

  file_path = filepath.Join("CONST", sub_dir, "DATA.FIL")

  file, err = os.Open(file_path)
  if err != nil {
    fmt.Println("Fatal: ", err)
    panic(err)
  }
  defer file.Close()

  fmt.Println("*****", sub_dir)
  index := GetIffIndex(sub_dir, file)
  fmt.Println("The index:\n", index, "\n")
  for _, ind_item := range index {
    fmt.Println("filename:", ind_item.filename, "\n", "offset", ind_item.offset, "length", ind_item.length, "i_start", ind_item.start,"i_end",ind_item.end)
  }

  out_dir := filepath.Join("output", sub_dir)

  err = os.MkdirAll(out_dir, os.ModePerm)
  if err != nil {
    fmt.Println("Failed to make directory ", out_dir)
    panic(err)
  }

  dir_info, err := os.Stat(out_dir)
  if err != nil {
    fmt.Println("Failed to stat directory ", out_dir)
    panic(err)
  }

  fmt.Println("Made ", out_dir, dir_info)

  file_info, err := file.Stat()
  if err != nil {
    fmt.Println("Failed to get info about the archive file", err)
    panic(err)
  }

  for entry_i, item := range index {
    if item.length < 1 {
      continue
    }

    fmt.Println("item ", item)
    entry := make([]byte, item.length)

    if item.offset + item.length > uint32(file_info.Size()) {
      fmt.Println("Index is incorrectly decrypted or corrupted, stopping extraction for this file")
      return
    }

    /*
     Trimming space is vital here because the file's strings are
     null-terminated and these characters cause a cryptic
     invalid argument error in later OpenFile and Write calls
     if they are not removed
    */
    out_path := filepath.Join(out_dir, strings.TrimRight(item.filename, "\x00"))

    out_file, err := os.OpenFile(out_path, os.O_TRUNC|os.O_CREATE|os.O_WRONLY, 0644)
    if err != nil {
      fmt.Println("Failed to create ", out_path)
      panic(err)
    }
    defer out_file.Close()


    _, err = file.Seek(int64(item.offset), 0)
    if err != nil {
      fmt.Println("Failed to seek to offset", item.offset, "for entry", item.filename)
      panic(err)
    }

    _, err = file.Read(entry)
    if err != nil {
      fmt.Println("Failed to read entry ", entry_i)
      panic(err)
    }

    _, err = out_file.Write(entry)
    if err != nil {
      fmt.Println("Failed to write file ", out_path, " for entry ", entry_i)
      panic(err)
    }

    fmt.Println("Extracted ", out_path)
  }
}

func GetLevelIndex(level_file *os.File) (map[string]*LevelIndexItem) {
  var err error

  /*
  The whole file is in IFF format, wrapped in a FORM chunk with a MAPS sub-header.
  This data is unimportant for further parsing, so skip past it.
  */
  _, err = level_file.Seek(12, 0)
  if err != nil {
    fmt.Println("Failed to seek past header in level file")
    panic(err)
  }

  index := make(map[string]*LevelIndexItem)
  position := 12

  for level_i := 0; level_i < 6; level_i++ {
    chunk_id := make([]byte, 4)
    chunk_length := make([]byte, 4)
    index_item := new(LevelIndexItem)

    _, err = level_file.Read(chunk_id)
    if err != nil {
      fmt.Println("Failed to read chunk id", level_i, "in level file")
      panic(err)
    }
    index_item.id = string(chunk_id)

    _, err = level_file.Read(chunk_length)
    if err != nil {
      fmt.Println("Failed to read data length for chunk ", level_i, "in level file")
      panic(err)
    }
    index_item.length = binary.BigEndian.Uint32(chunk_length)

    position += 8
    index_item.offset = uint32(position)

    _, err = level_file.Seek(int64(index_item.length), 1)
    if err != nil {
      fmt.Println("Failed to seek past chunk ", level_i, "in level file")
      panic(err)
    }

    index[index_item.id] = index_item

    // 4 characters + uin32 + chunk length
    position += (8 + int(index_item.length))
  }

  return index
}

func GetLevelColorMap(level_file *os.File) ([]ColorMapEntry) {
  var err error

  level_index := GetLevelIndex(level_file)

  color_map := make([]ColorMapEntry, 256)
  _, err = level_file.Seek(int64(level_index["CMAP"].offset), 0)
  if err != nil {
    fmt.Println("Failed to open level file")
    panic(err)
  }

  colors := make([]byte, 3)
  for color_i := range(color_map) {
    _, err:= level_file.Read(colors)
    if err != nil {
      fmt.Println("Failed to open level file")
      panic(err)
    }
    color_map[color_i].r = colors[0]
    color_map[color_i].g = colors[1]
    color_map[color_i].b = colors[2]
  }

  return color_map
}

func ExtractSprite(sprite_file *os.File, out_path string, info SpriteItem, cmap []ColorMapEntry) {
  //fmt.Println("trying to draw this sprite", info)

  var err error
  /*
  There are two uint16's at the start of the sprite
  header whose value is unknown, so seek past them
  */
  _, err = sprite_file.Seek(int64(info.offset) + 4, 0)
  if err != nil {
    fmt.Println("Failed to seek to offset", info.offset, "in sprite file")
    panic(err)
  }

  width_buf := make([]byte, 2)
  height_buf := make([]byte, 2)

  _, err = sprite_file.Read(width_buf)
  if err != nil {
    fmt.Println("Failed to read sprite width")
    panic(err)
  }

  _, err = sprite_file.Read(height_buf)
  if err != nil {
    fmt.Println("Failed to read sprite height")
    panic(err)
  }

  out_file, err := os.OpenFile(out_path, os.O_TRUNC|os.O_CREATE|os.O_WRONLY, 0644)
  if err != nil {
    fmt.Println("Failed to create output image", out_path)
    panic(err)
  }

  //width := int(binary.BigEndian.Uint16(width_buf))
  //height := int(binary.BigEndian.Uint16(height_buf))
  width := int(binary.LittleEndian.Uint16(width_buf))
  height := int(binary.LittleEndian.Uint16(height_buf))
  fmt.Println("Image is ", width, "x", height)

  imgRect := image.Rect(0, 0, width, height)
  img := image.NewNRGBA(imgRect)

  var last_was_zero bool

  pixels := make([]ColorMapEntry, width * height)
  pixel_buf := make([]byte, info.length)

  _, err = sprite_file.Read(pixel_buf)
  if err != nil {
    fmt.Println("Failed to read sprite height")
    panic(err)
  }

  // lookup the color values and decompress transparency
  var trans_entry ColorMapEntry
  trans_entry.transparent = true

  for pixel_i := range pixel_buf {
    cmap_key := uint8(pixel_buf[pixel_i])
    if cmap_key == 0 {
      last_was_zero = true
      continue
    }

    if last_was_zero {
      /*
      Transparency is RLE-encoded: a 0 followed by
      a number indicating how many transparent pixels
      are there.
      */

      last_was_zero = false
      for zero_i := 0; zero_i < int(cmap_key); zero_i++ {
        fill_i := (pixel_i - 1) + zero_i
        if fill_i > len(pixels) - 1 {
          break
        }
        pixels[fill_i] = trans_entry
      }
      pixel_i += int(cmap_key)
      continue
    }

    if pixel_i > len(pixels) - 1 {
      break
    }

    pixels[pixel_i] = cmap[cmap_key]
  }
  fmt.Println("the pixels", pixels)

  //fmt.Println("processed pixels", pixels)

  px_count := -1
  for x_idx := 0; x_idx < height; x_idx++ {
    for y_idx := 0; y_idx < height; y_idx++ {
      px_count++

      if px_count > len(pixels) - 1 {
        break
      }

      if pixels[px_count].transparent {
        continue
      }

      rgb := new(color.NRGBA)
      rgb.R = pixels[px_count].r
      rgb.G = pixels[px_count].g
      rgb.B = pixels[px_count].b
      rgb.A = 0xFF

      fmt.Println("x", x_idx, "y", y_idx, "RGBA", rgb)
      img.SetNRGBA(x_idx, y_idx, *rgb)
    }
  }

  fmt.Println("the image", img.Rect)

  err = png.Encode(out_file, img)
  if err != nil {
    fmt.Println("Failed to write sprite png")
    panic(err)
  }

  fmt.Println("wrote", out_path)
}

func ExtractSprites(filename string, cmap []ColorMapEntry) {
  var err error

  name_bits := strings.Split(filename, ".")
  out_dir := name_bits[0]

  err = os.MkdirAll(out_dir, os.ModePerm)
  if err != nil {
    fmt.Println("Failed to make directory ", out_dir)
    panic(err)
  }

  sprite_file, err := os.Open(filename)
  if err != nil {
    fmt.Println("Failed to open sprite file", filename)
    panic(err)
  }
  defer sprite_file.Close()

  count_buf := make([]byte, 4)
  _, err = sprite_file.Read(count_buf)
  if err != nil {
    fmt.Println("Failed to read sprite count in", filename)
    panic(err)
  }

  file_info, err := sprite_file.Stat()
  if err != nil {
    fmt.Println("Failed to stat sprite file: ", err)
    panic(err)
  }
  sprite_file_size := file_info.Size()

  num_sprites := binary.LittleEndian.Uint32(count_buf)
  fmt.Println(filename, "has", num_sprites, "sprites")

  offset_buf := make([]byte, 4)
  offsets := make([]SpriteItem, num_sprites)

  // read in the offsets for all sprites in the file
  // this is the worst code ever ;(
  for offset_i := 0; offset_i < int(num_sprites); offset_i++ {
    sprite_file.Read(offset_buf)
    if err != nil {
      fmt.Println("Failed to read offset", offset_i, "in", filename)
      panic(err)
    }

    this_offset := binary.LittleEndian.Uint32(offset_buf)
    if this_offset < 1 {
      // blank entry, can be ignored
      continue
    }

    if int64(this_offset) > sprite_file_size {
      fmt.Println("Offset", this_offset, "is bigger than the file which is ", sprite_file_size)
      panic(this_offset)
    }

    offsets[offset_i].offset = this_offset
  }

  // infer the length of each sprite using the loaded offsets
  sprite_pt := 0
  for offset_i := 0; offset_i < int(num_sprites); offset_i++ {
    // last offset in file, just read until the end
    if (offset_i + 1) == int(num_sprites) {
      offsets[offset_i].length = uint32(file_info.Size()) - offsets[offset_i].offset
      continue
    }

    if offsets[offset_i + 1].offset == 0 {
      // find the next non-zero offset
      sprite_pt = offset_i

      for offsets[sprite_pt].offset == 0 {
        sprite_pt++;
        if sprite_pt + 1 == int(num_sprites) {
          break
        }
      }

      if sprite_pt + 1 == int(num_sprites) {
        // there are only 0 offsets remaining in the file, just
        // read until the end
        offsets[offset_i].length = uint32(file_info.Size()) - offsets[offset_i].offset

        continue
      }

      // use the next non-zero offset we found to calculate the length
      offsets[offset_i].length = offsets[sprite_pt].offset - offsets[offset_i].offset
    } else {
      offsets[offset_i].length = offsets[offset_i + 1].offset - offsets[offset_i].offset
    }
  }

  fmt.Println("sprite offsets", offsets)

  //for sprite_i := 0; sprite_i < int(num_sprites); sprite_i++ {
    //im_filename := fmt.Sprintf("%d.png", sprite_i)
    //im_path := filepath.Join(out_dir, im_filename)

    //if offsets[sprite_i].offset == 0 {
       //the file has empty entries or placeholders?
      //continue
    //}

    //ExtractSprite(sprite_file, im_path, offsets[sprite_i], cmap)
  //}
  im_filename := fmt.Sprintf("%d.png", 0)
  im_path := filepath.Join(out_dir, im_filename)
  ExtractSprite(sprite_file, im_path, offsets[2], cmap)
}

func main() {
  dirs := []string{"GFX", "SFX", "LFX", "DATA"}

  for _, sub_dir := range dirs {
    ExtractFile(sub_dir)
  }

  //level_file, err := os.Open("output/DATA/LEV01.DAT")
  //if err != nil {
    //fmt.Println("Failed to open level file")
    //panic(err)
  //}
  //defer level_file.Close()

  //sprite_files, err := filepath.Glob("output/DATA/*.SPR")
  //if err != nil {
    //fmt.Println("Failed to glob sprite files")
    //panic(err)
  //}

  //cmap := GetLevelColorMap(level_file)

  //for _, sprite_filename := range sprite_files {
    //ExtractSprites(sprite_filename, cmap)
  //}
}
