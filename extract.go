package main

import (
  "bufio"
  "encoding/binary"
  "fmt"
  "os"
  "strings"
  "io/ioutil"
  "log"
  "path/filepath"
)

type IndexItem struct {
  filename string
  offset uint32
  length uint32
  start uint32
  end uint32
}

func getIffIndex(sub_dir string, fil_file *os.File) ([]IndexItem) {
  // get the encrypted value for the number of entries in the file
  var (
    num_entries uint32
    err error
  )

  buffer := bufio.NewReader(fil_file)
  enc_entries := make([]byte, 4)

  file_info, err := file.Stat()
  if err != nil {
    fmt.Println("Fatal: ", err)
    log.Fatal(err)
  }

  _, err = buffer.Read(enc_entries)
  if err != nil {
    fmt.Println("Fatal: ", err)
    log.Fatal(err)
  }

  // decrypt the value to get the number of entries
  num_entries = binary.LittleEndian.Uint32(enc_entries) ^ 0x3BD7A59A
  fmt.Println("entries: ", num_entries)

  index_buf := make([]byte, 17 * num_entries)
  index := make([]IndexItem, num_entries)

  _, err = buffer.Read(index_buf)
  if err != nil {
    fmt.Println("Fatal: ", err)
    log.Fatal(err)
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

func extractFile(sub_dir string) {
  var (
    file *os.File
    err error
    file_path string
  )

  file_path = filepath.Join("CONST", sub_dir, "DATA.FIL")

  file, err = os.Open(file_path)
  if err != nil {
    fmt.Println("Fatal: ", err)
    log.Fatal(err)
  }

  defer file.Close()

  fmt.Println("*****", sub_dir)
  index := getIffIndex(sub_dir, file)
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
    log.Fatal(err)
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
      log.Fatal(err)
    }

    fmt.Println("Extracted ", out_path)
  }
}

func main() {
  dirs := []string{"GFX", "SFX", "LFX", "DATA"}

  for _, sub_dir := range dirs {
    extractFile(sub_dir)
  }
}
