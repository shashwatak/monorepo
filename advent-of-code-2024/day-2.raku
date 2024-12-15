
sub sign($val) {
  return $val <=> 0;
}

sub check_diffs($diff, $prev_diff) {

  my $dir = sign($diff);
  if $prev_diff && sign($prev_diff) != $dir {
    # say "dir change: $prev_diff, $diff";
    return False;
  }
  if (abs($diff) < 1 || abs($diff) > 3) || ($diff == 0) {
    # say "too much change $diff";
    return False;
  }
  return True;

}

### spent a long time trying the non-brute force way kept getting 628

sub analyze(@report, $damper=False) {

  my $can_skip = $damper;
  my $prev_diff;

  for @report.kv -> $i, $val {

    last if $i + 2 == @report.elems && $can_skip;

    last if $i + 1 == @report.elems;

    my $next = @report[$i + 1];
    my $diff = $next - $val;
    if !check_diffs($diff, $prev_diff) {
      if $can_skip {
        $can_skip = False;
        next;
      }
      return False;
    }

    $prev_diff = $diff;

  }

  return True;
}

sub brute_force(@report) {
  my $count = @report.elems;
  return True if analyze(@report);
  for 0..^$count-1 -> $i {
    my @skipped_i = |@report[0..^$i], |@report[$i+1..^$count];
    return True if analyze(@skipped_i);
  }

  return True if analyze(@report[0..^$count-1]);

  return False;
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

sub smarter(@report) {
    my @diffs = diffs(@report);
    my $abs = all(@diffs.map(*.abs));
    my $dirs = set(@diffs.map(*.sign));
    return $abs <= 3 && $abs >= 1 && $dirs.elems == 1;
}



my @tests = [
          [7, 6, 4, 2, 1], # Safe without removing any level.
          [1, 2, 7, 8, 9], # Unsafe regardless of which level is removed.
          [9, 7, 6, 2, 1], # Unsafe regardless of which level is removed.
          [1, 3, 2, 4, 5], # Safe by removing the second level, 3.
          [8, 6, 4, 4, 1], # Safe by removing the third level, 4.
          [1, 3, 6, 7, 9], # Safe without removing any level.
          [42, 41, 39, 36, 37, 32], # weird
        ]; 

for @tests -> @test {
  analyze(@test);
}

for @tests -> @test {
  analyze(@test, True);
}

for @tests -> @test {
  brute_force(@test);
}

for @tests -> @test {
  smarter(@test);
}

my $count = 0;
my $countSkip = 0;
my $countBrute = 0;
my $countSmart = 0;

my @reports = (open 'day-2-input.txt').lines.map(*.split(' ').map(*.Int));

for @reports -> @report {
  if analyze(@report) {$count++;}
  if analyze(@report, True) {$countSkip++;}
  if brute_force(@report) {$countBrute++;}
  if smarter(@report) {$countSmart++;}

}

say $count;
say $countSkip;
say $countBrute;
say $countSmart;


## These were the ones analyze w/ skip got wrong
# (9 12 9 11 14 16 17 20)True
# (53 56 57 60 61 65 66)True
# (30 32 39 42 45)True
# (12 10 13 16 19 21 22)False
# (68 67 69 67 64)True
# (78 76 80 81 84 87)False
# (48 52 50 51 54)False
# (22 19 16 14 11 14 11)True
# (39 38 34 32 30)True
# (37 36 35 34 30 33)False
# (62 60 58 52 50 47)True
# (15 14 12 6 9)False
# (24 25 24 23 21 19 18 17)False
# (2 4 5 9 11)True
# (73 70 68 67 62 61 58 56)True
# (88 90 88 86 84 82 80)False
# (20 19 16 13 12 9 5 3)True
# (66 68 64 61 58 57 54)False
# (71 74 75 78 85 87 90 91)True
# (3 8 6 8 10 12 15)False


