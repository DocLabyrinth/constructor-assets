require 'bindata'

class Image < BinData::Record
  endian :little
  uint16 :unk1
  uint16 :unk2
  uint16 :width
  uint16 :height
  array :pixels, :type => :uint8, :read_until => :eof
end

class ImFile < BinData::Record
  endian :little
  uint32 :spr_count
  array :offsets, :type => :uint32, :initial_length => :spr_count
end

module Level
  class Cmap < BinData::Record
    endian :big
    array :colors, :initial_length => 256 do
      uint8 :red
      uint8 :green
      uint8 :blue
    end
  end

  class Blocks < BinData::Record
    endian :big
    array :iso_blocks, :read_until => :eof do
      string :block, :length => 256
    end
  end

  class File < BinData::Record
    endian :big
    array :chunks, :initial_length => 6 do
      string :id, :length => 4
      uint32 :chunk_size
      #choice :data, :selection => :id, :choices => {
        #'CMAP' => :cmap,
        #'BLKS' => :blocks,
        #:default => :string,
      #}
      string :data, :length => :chunk_size
    end
  end
end

module Archive
  class File < BinData::Record
    endian :big
    uint32 :num_entries
  end

end
