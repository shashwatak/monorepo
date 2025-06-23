#!/bin/bash

CSV="/home/autod/monorepo/solvers/bazel-out/k8-fastbuild/bin/osqp.runfiles/_main/path.csv"
OBSTACLE="/home/autod/monorepo/solvers/bazel-out/k8-fastbuild/bin/osqp.runfiles/_main/obstacle.csv"
OUT="path.png"

# Extract obstacle parameters
read CENTER_X CENTER_Y HALF_X HALF_Y < <(tail -n 1 "$OBSTACLE" | awk -F, '{print $1, $2, $3, $4}')

# Calculate box corners for plotting
X1=$(echo "$CENTER_X - $HALF_X" | bc -l)
X2=$(echo "$CENTER_X + $HALF_X" | bc -l)
Y1=$(echo "$CENTER_Y - $HALF_Y" | bc -l)
Y2=$(echo "$CENTER_Y + $HALF_Y" | bc -l)

gnuplot <<EOF
set datafile separator ","
set terminal pngcairo size 800,600
set output "${OUT}"
set title "Optimized Path with Obstacle"
set xlabel "x"
set ylabel "y"
set grid
set style fill transparent solid 0.2 noborder
set object 1 rect from ${X1},${Y1} to ${X2},${Y2} fc rgb "red" fillstyle solid 0.2 border lc rgb "red"
plot "${CSV}" using 1:2 with linespoints title "Path"
EOF

echo "Plot saved to ${OUT}"
