my @instructions = (open 'day-3-input.txt').comb(/mul\(\d**1..3\,\d**1..3\)/);

sub multiply($instruction) {
  my $xy = $instruction.match(/(\d*)\,(\d*)/);
  return $xy[0] * $xy[1];
}

say [+] @instructions.map(&multiply);
