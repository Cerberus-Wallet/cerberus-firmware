#!/usr/bin/perl
print <<'EOF';
/*
 * This file is part of the Cerberus project, https://cerberus.uraanai.com/
 *
 * Copyright (C) 2016 Jochen Hoenicke <hoenicke@gmail.com>
 *
 * This library is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this library.  If not, see <http://www.gnu.org/licenses/>.
 */

EOF

my @arr1;
my @arr2;
my $x = 0;
my $l = "00";
my @words;
while (<>) {
    $_ =~ /([1-9]{2})[1-9] ([1-6]):(.*)/;
    my $n = $1;
    my $c = $2;
    my @nw = split(",", $3);
    die if $c != @nw;
    die if $c > 6;
    push @words, @nw;
    if ($n ne $l) {
        $len = @arr2;
        die if $len - $arr1[-1] > 9;
        push @arr1, $len;
    }
    push @arr2, $x;
    $x += $c;
    $l = $n;
}
$len = @arr2;
push @arr1, $len;
push @arr2, $x;

sub computerange($$$) {
    my ($i1, $i2, $entries) = @_;
    $prev = $i1 == 0 ? "_" : $words[$i1 - 1];
    $first = $words[$i1];
    $last = $words[$i2-1];
    $next = $i2 == @words ? "_" : $words[$i2];
    my $j;
    for ($j = 0; $j < 5; $j++) {
        last if substr($first, 0, $j+1) ne substr($last, 0, $j+1);
        last if substr($prev, 0, $j) ne substr($first, 0, $j)
             && substr($next, 0, $j) ne substr($last, 0, $j);
    }
    $prefix = substr($first, 0, $j);
    $range = "";
    $rng = 0;
    if (substr($prev, 0, $j) eq substr($first, 0, $j)
        || substr($last, 0, $j) eq substr($next, 0, $j)) {
        $range = "[".substr($first, $j, 1) . "-". substr($last, $j, 1)."]";
        $rng++;
        if ($j <= 1) {
            $range = substr($first,0, $j+1)."-".substr($last,0,$j+1);
            $prefix= "";
        }
    }
    if (substr($prev, 0, $j+1) eq substr($first, 0, $j+1)
        || substr($last, 0, $j+1) eq substr($next, 0, $j+1)) {
        $j = 0; $rng = 2;
    }
    #printf STDERR "  # %1d: %9s - %9s = \U$prefix$range\E\n", $entries, $first, $last;
    return $j + $rng;
}

print << 'EOF';
/* DO NOT EDIT: This file is automatically generated by
 * cd ../gen/wordlist
 * perl build-recoverytable.pl recovery_english.txt
 */

EOF

$len = @arr1;
print "static const uint16_t word_table1[$len] =\n";
print "{";
for ($i = 0; $i< @arr1; $i++) {
    print "\n   " if ($i % 9 == 0);
    $prefixlen = computerange($arr2[$arr1[$i]], $arr2[$arr1[$i+1]], $arr1[$i+1]-$arr1[$i]);
    $prefixlen = 0 if ($i == @arr1 - 1);
    printf(" %5d,", $arr1[$i] + 4096 * $prefixlen);
}
print "\n};\n\n";

$len = @arr2;
print "static const uint16_t word_table2[$len] =\n";
print "{";
for ($i = 0; $i< @arr2; $i++) {
    print "\n   " if ($i % 9 == 0);
    $prefixlen = computerange($arr2[$i], $arr2[$i+1], $arr2[$i+1]-$arr2[$i]);
    $prefixlen = 0 if ($i == @arr2 - 1);
    printf(" %5d,", $arr2[$i] + 4096 * $prefixlen);
}
print "\n};\n";
