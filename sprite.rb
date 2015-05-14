require 'bundler/setup'
require 'json'
require 'rmagick'
require_relative 'defs'
require 'timeout'

out_dir = 'output/DATA'

File.open("#{out_dir}/LEV01.DAT", 'rb') do |f|
  f.seek(12)
  level_file = Level::File.read(f)
  level_file.chunks.each do |chunk|
    File.open("#{out_dir}/LEV01-#{chunk.id}.DAT", 'wb') do |of|
      of.write(chunk.data.to_binary_s)
    end
  end
end

File.open("#{out_dir}/LEV01-BLKS.DAT", 'rb') do |f|
  blks = Level::Blocks.read(f)

  sprite_dir = "#{out_dir}/BLKS"
  begin
    Dir.mkdir(sprite_dir)
  rescue Errno::EEXIST
  end

  cmap = Level::Cmap.read(File.read("#{out_dir}/LEV01-CMAP.DAT"))
  block_width = 32
  block_height = 15

  # see https://github.com/shlainn/game-file-formats/wiki/Constructor-level-files
  block_mask = [
    "..............XXXX..............",
    "............XXXXXXXX............",
    "..........XXXXXXXXXXXX..........",
    "........XXXXXXXXXXXXXXXX........",
    "......XXXXXXXXXXXXXXXXXXXX......",
    "....XXXXXXXXXXXXXXXXXXXXXXXX....",
    "..XXXXXXXXXXXXXXXXXXXXXXXXXXXX..",
    "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "..XXXXXXXXXXXXXXXXXXXXXXXXXXXX..",
    "....XXXXXXXXXXXXXXXXXXXXXXXX....",
    "......XXXXXXXXXXXXXXXXXXXX......",
    "........XXXXXXXXXXXXXXXX........",
    "..........XXXXXXXXXXXX..........",
    "............XXXXXXXX............",
    "..............XXXX.............."
  ].map{|line| line.split("")}

  puts "Extracting isometric tiles as individual images"

  blks.iso_blocks.each_with_index do |block, index|
    img = Magick::Image.new(block_width, block_height)
    img.alpha(Magick::TransparentAlphaChannel)

    px_count = -1

    # the blocks are all 32x15 images, with each pixel
    # represented as a reference to a value in the colour
    # palette like the other sprites
    for y_it in 0..(block_height-1)
      for x_it in 0..(block_width-1)
        next unless block_mask[y_it][x_it] == 'X'

        px_count += 1

        curr_pix = cmap.colors[block[px_count]]
        rgb_str = "rgb(#{curr_pix["red"]}, #{curr_pix["green"]}, #{curr_pix["blue"]})"
        img.pixel_color(x_it, y_it, rgb_str)
      end
    end

    img.write("#{sprite_dir}/#{index}.png")
  end
end


def project(x, y)
    proj_angle = Math::PI / 6

    {
      :x => (x - y) * Math.cos(proj_angle),
      :y => (x + y) * Math.sin(proj_angle),
    }
end

File.open("#{out_dir}/LEV01-MAPD.DAT", 'rb') do |f|
  mapd = Level::Mapd.read(f)
  tiles = mapd.tiles.map do |tile_int|
    out = {:id => tile_int & 0x0FFF}
    out[:flip_x] = true if tile_int & 0x1000 > 0
    out[:flip_y] = true if tile_int & 0x2000 > 0
  end

  block_w = 32
  block_h = 15

  im_width = mapd.width * block_w
  im_height = mapd.height * block_h

  map_img = Magick::Image.new(im_width, im_height)
  map_img.alpha(Magick::TransparentAlphaChannel)

  map = {}

  puts "Ordering map tiles as a large jpg"

  tile_count = -1
  for y_it in 0..(mapd.height-1)
    for x_it in 0..(mapd.width-1)
      tile_count += 1

      tile_int = mapd.tiles[tile_count]
      tile_id = tile_int & 0x0FFF
      next if tile_id == 0

      tile = {:id => tile_id}
      tile[:flip_x] = true if tile_int & 0x1000 > 0
      tile[:flip_y] = true if tile_int & 0x2000 > 0

      tile_img = Magick::Image.read("#{out_dir}/BLKS/#{tile_id}.png").first
      tile_img.flop! if tile[:flip_x]
      tile_img.flip! if tile[:flip_y]
      proj = project((x_it * block_w/2), (y_it * (block_h+1)))
      map_img.composite!(tile_img, proj[:x] + (im_width/2), proj[:y], Magick::OverCompositeOp)
      tile_img.destroy!

      map["#{x_it}x#{y_it}"] = tile
    end
  end

  map_img.trim!.write("#{out_dir}/LEV01-blocks-combined.jpg")

  json = {
    :width => mapd.width,
    :height => mapd.height,
    :tiles => map
  }

  File.open("#{out_dir}/mapd-extract.json", "w") do |f|
    f.write(JSON.dump(json))
  end
end

Dir["#{out_dir}/*.SPR"].each do |filename|
  sprite_dir_name = filename.split('/').last.split('.').first
  sprite_dir = "#{out_dir}/#{sprite_dir_name}"
  begin
    Dir.mkdir(sprite_dir)
  rescue Errno::EEXIST
  end

  puts "Extracting sprites as individual pngs"

  File.open(filename, 'rb') do |f|

    cmap = Level::Cmap.read(File.read("#{out_dir}/LEV01-CMAP.DAT"))

    imf = ImFile.read(f)
    puts "Sprites in file #{filename}: #{imf.spr_count}"
    offsets = imf.offsets.to_a.delete_if{|o| o < 1}.unshift(0)

    offsets.each_with_index do |offset, index|
      f.seek(offset)
      next_offset = offsets[index + 1]
      data_length = next_offset.nil? ? offset : (next_offset - offset)
      im = Image.read(f.read(data_length))

      next if im.width < 1 || im.height < 1

      last_was_zero = false
      color_arr = []

      im.pixels.each do |pixel|
        #a 0 means the number after it indicates how
        #many pixels should be transparent as a means
        #of compressing empty pixels
        if pixel == 0
          last_was_zero = true
          next
        end

        if last_was_zero
          color_arr = color_arr.concat([nil] * pixel)
          last_was_zero = false
          next
        end

        #each pixel is a number indicating a position
        #in the color palette for the level
        color_arr.push(cmap.colors[pixel])
      end

      img = Magick::Image.new(im.width, im.height)
      img.alpha(Magick::TransparentAlphaChannel)

      px_count = -1

      for y_it in 0..im.height-1
        for x_it in 0..im.width-1
          px_count += 1
          next if color_arr[px_count].nil?

          curr_pix = color_arr[px_count]
          rgb_str = "rgb(#{curr_pix["red"]}, #{curr_pix["green"]}, #{curr_pix["blue"]})"
          img.pixel_color(x_it, y_it, rgb_str)
        end
      end

      img.write("#{sprite_dir}/#{index}.png")
    end
  end
end

