my @rows = (open 'day-4-small-input.txt').lines.map(*.comb());
say @rows;
my @cols = []; 
for ^@rows[0].elems -> $i {
  say "$i";
  @cols.append();
}
say @cols;
