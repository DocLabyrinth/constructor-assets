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
    endian :little
    uint32 :enc_num
    uint32 :num, :value => lambda { enc_num ^ 0x3BD7A59A }

    # repeat the lambda because something about the order
    # of processing in BinData means the encrypted value
    # still gets passed to :initial_length
    array :i_index, :initial_length => lambda { enc_num ^ 0x3BD7A59A } do
      string :raw, :length => 17
      string :filename, :value => lambda {
        raw.each_byte.each_with_index.map do |b, index|
          force = BinData::Bit4.read(b)
          force -= 39
          force ^= 0xA5
          force -= (27 + index)
          puts '---------'
        end.map(&:chr)
      }
      #uint32 :i_offset, :value => lambda {
        #raw[13,4].each_byte.each_with_index.map do |b, index|
          #b -= 39
          #b ^= 0xA5
          #b -= (27 + index)
          #b
        #end
      #}
    end
  end
end
