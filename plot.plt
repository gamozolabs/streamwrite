set term wxt size 1440,900 persist
set logscale x
set xtics 16
set title "x86 single core sequeltial memory write performance"
set xlabel "Block size"
set ylabel "GiB/sec"
set arrow from 48 * 1024,graph(0,0) to 48 * 1024,graph(1,1) nohead
set arrow from 1280 * 1024,graph(0,0) to 1280 * 1024,graph(1,1) nohead
set arrow from 18 * 1024 * 1024,graph(0,0) to 18 * 1024 * 1024,graph(1,1) nohead
plot "log.txt" u 1:2 w l t "Volatile write i32", "log.txt" u 1:3 w l t "Non-temporal write i32"

