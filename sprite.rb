require 'bundler/setup'
require 'rmagick'
require_relative 'defs'

File.open('CONST/GFX/DATA.FIL', 'rb') do |f|
  archive = Archive::File.read(f)
  puts "entries in file: #{archive.num}"
  p archive.i_index
end

#File.open('LEV01.DAT', 'rb') do |f|
  #f.seek(12)
  #level_file = Level::File.read(f)
  #level_file.chunks.each do |chunk|
    #File.open("LEV01-#{chunk.id}.DAT", 'wb') do |of|
      #of.write(chunk.data.to_binary_s)
    #end
  #end
#end

#File.open('LEV01-CMAP.DAT', 'rb') do |f|
  #cmap = Level::Cmap.read(f)
#end

#File.open('LEV01-BLKS.DAT', 'rb') do |f|
  #blks = Level::Blocks.read(f)
  #p blks.snapshot
#end

#File.open('HOUSES.SPR', 'rb') do |f|

  #cmap = Level::Cmap.read(File.read('LEV01-CMAP.DAT'))

  #imf = ImFile.read(f)
  #puts "Sprites in file: #{imf.spr_count}"
  #offsets = imf.offsets.to_a.delete_if{|o| o < 1}.unshift(0)

  #offsets.each_with_index do |offset, index|
    #f.seek(offset)
    #next_offset = offsets[index + 1]
    #data_length = next_offset.nil? ? offset : (next_offset - offset)
    #im = Image.read(f.read(data_length))

    #next if im.width < 1 || im.height < 1

    #puts "Im: #{im.width}, #{im.height}"

    #last_was_zero = false
    #color_arr = []

    #im.pixels.each do |pixel|
       #a 0 means the number after it indicates how
       #many pixels should be transparent as a means
       #of compressing empty pixels
      #if pixel == 0
        #last_was_zero = true
        #next
      #end

      #if last_was_zero
        #color_arr = color_arr.concat([nil] * pixel)
        #last_was_zero = false
        #next
      #end

       #each pixel is a number indicating a position
       #in the color palette for the level
      #color_arr.push(cmap.colors[pixel])
    #end

    #img = Magick::Image.new(im.width, im.height)
    #img.background_color = 'Transparent'

    #px_count = -1

    #for y_it in 0..im.height-1
      #for x_it in 0..im.width-1
        #px_count += 1
        #next if color_arr[px_count].nil?

        #curr_pix = color_arr[px_count]
        #rgb_str = "rgb(#{curr_pix["red"]}, #{curr_pix["green"]}, #{curr_pix["blue"]})"
        #img.pixel_color(x_it, y_it, rgb_str)
      #end
    #end

    #img.write("houses/#{index}.png")
  #end
#end
