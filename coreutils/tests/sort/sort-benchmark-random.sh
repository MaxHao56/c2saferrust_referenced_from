#!/bin/sh
# Benchmark sort on randomly generated data.

# Copyright (C) 2010-2024 Free Software Foundation, Inc.

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# Written by Glen Lenker.

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
. "$SCRIPTPATH/../../tests/init.sh"; path_prepend_ $1
require_perl_

very_expensive_

$PERL -e '
my $num_lines = 500000;
my $length = 100;

for (my $i=0; $i < $num_lines; $i++)
{
    for (my $j=0; $j < $length; $j++)
    {
      printf "%c", 32 + rand(94);
    }
    print "\n";
}' > in || framework_failure_

# We need to generate a lot of data for sort to show a noticeable
# improvement in performance. Sorting it in PERL may take awhile.

$PERL -e '
open (FILE, "<in");
my @list = <FILE>;
print sort(@list);
close (FILE);
' > exp || framework_failure_

time sort in > out || fail=1

compare exp out || fail=1

Exit $fail
