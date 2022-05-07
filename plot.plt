set term wxt size 1440,900 persist
set logscale x
set xtics ("64 bytes" 64, "512 bytes" 512, "4 KiB" 4096, "48 KiB (L1)" 48 * 1024, "256 KiB" 256 * 1024, "1.25 MiB (L2)" 1280 * 1024, "4 MiB" 4 * 1024 * 1024, "8 MiB" 8 * 1024 * 1024, "18 MiB (L3)" 18 * 1024 * 1024, "64 MiB" 64 * 1024 * 1024, "1 GiB" 1024 * 1024 * 1024, "4 GiB" 4 * 1024 * 1024 * 1024, "32 GiB" 31.8 * 1024 * 1024 * 1024)
set ytics 1
set title "Single core sequential \\'i32\\' memory write throughput | Intel(R) Xeon(R) Silver 4310 CPU \\@ 2.10GHz | 2667 MHz 8 memory channels | Turbo on | HT on | Affine to single HW thread"
set xlabel "Block size"
set ylabel "GiB/sec"
set grid mxtics mytics xtics ytics
set arrow from 48 * 1024,graph(0,0) to 48 * 1024,graph(1,1) nohead
set arrow from 1280 * 1024,graph(0,0) to 1280 * 1024,graph(1,1) nohead
set arrow from 18 * 1024 * 1024,graph(0,0) to 18 * 1024 * 1024,graph(1,1) nohead
plot "log_physcpubind_0_membind_0.txt" u 1:2 w l t "Volatile write i32 node local" lc 1, "log_physcpubind_0_membind_0.txt" u 1:3 w l t "Non-temporal write i32 node local" dashtype 4 lc 1, \
    "log_physcpubind_0_membind_1.txt" u 1:2 w l t "Volatile write i32 node remote" lc 5, "log_physcpubind_0_membind_1.txt" u 1:3 w l t "Non-temporal write i32 node remote" dashtype 4 lc 5, \
    "log_physcpubind_0_interleave_0_1.txt" u 1:2 w l t "Volatile write i32 node interleave" lc 4, "log_physcpubind_0_interleave_0_1.txt" u 1:3 w l t "Non-temporal write i32 node interleave" dashtype 4 lc 4

