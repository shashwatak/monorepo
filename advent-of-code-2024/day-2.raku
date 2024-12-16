
sub sign($val) {
  return $val <=> 0;
}


sub diffs(@report) {
  my @diffs = [];
  my $count = @report.elems;
  for 0..^$count-1 -> $i {
    my $diff = @report[$i+1] - @report[$i];
    @diffs.append($diff);
  }
  return @diffs;
}

sub analyze(@report) {
  my @diffs = diffs(@report);
  my $abs = all(@diffs.map(*.abs));
  my $dirs = set(@diffs.map(*.sign));
  return $abs <= 3 && $abs >= 1 && $dirs.elems == 1;
}


sub brute_force(@report) {
  my $count = @report.elems;
  return True if analyze(@report);
  return True if analyze(@report[0..^$count-1]);
  for 0..^$count-1 -> $i {
    my @skipped_i = |@report[0..^$i], |@report[$i+1..^$count];
    return True if analyze(@skipped_i);
  }


  return False;
}


my $count = 0;
my $countBrute = 0;

my @reports = (open 'day-2-input.txt').lines.map(*.split(' ').map(*.Int));

for @reports -> @report {
  if analyze(@report) {$count++;}
  if brute_force(@report) {$countBrute++;}

}

say $count;
say $countBrute;

