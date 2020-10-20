#!/usr/bin/ruby
# This generates blit patterns for building a blank onscreen keyboard

# RLE encodings for top row, letter keys, and spacebar row
fkey = [3, 65, 1, 65, 68, 65, 1, 65, 3]
letters = [3] + [32, 1]*4 + [32, 2, 32] + [1, 32]*4 + [3]
bottom = [36, 32, 1, 32, 1, 132, 1, 32, 1, 32, 36]

# Expand run length encoded array to 336 bits, pad to 352 bits
def expand_bits(rle)
  rle.zip(rle.each_index.to_a).map {|n, i| [i%2]*n}.flatten + [0]*16
end

# Pack 352 bits into 11 u32 hex literals
def hexify(bits)
  row = []
  for i in 0..10
    word = 0;
    for b in 0..31
      word = (word >> 1) | (bits.shift==1 ? 0 : 0x80000000)
    end
    row.push(word)
  end
  row.map {|v| "0x%08x" % v}
end

puts "// fkey: #{hexify(expand_bits(fkey)).join(", ")}"
puts "// letters: #{hexify(expand_bits(letters)).join(", ")}"
puts "// bottom: #{hexify(expand_bits(bottom)).join(", ")}"
