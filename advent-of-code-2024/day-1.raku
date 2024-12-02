# for (my $i, my $j) = (open 'day-1-input.txt').lines.map(*.split(' ').map(*.Int)) {
#   say "$i $j"
# }

my @a;
my @b;

for (open 'day-1-input.txt').lines.map(*.split('  ')) -> ($i, $j) {
  @a.push(Int($i));
  @b.push(Int($j));
}

@a .= sort;
@b .= sort;

my $dist = 0;
for @a Z @b -> ($i, $j) {
  my $d = abs($i - $j);
  
  say "abs($i - $j) = $d";

  $dist += $d;
}

say $dist;
