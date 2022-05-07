use std::alloc::Layout;

fn rdtsc() -> u64 {
    // Fences help a bit with determinism and stability of benchmarks, as
    // memory accesses can be moved after the rdtsc
    unsafe { core::arch::asm!("mfence"); }
    let ret = unsafe { core::arch::x86_64::_rdtsc() };
    unsafe { core::arch::asm!("mfence"); }
    ret
}

/// Maximum benchmark size, in bytes
const MAX_BENCH_SIZE: usize = 32 * 1024 * 1024 * 1024;

/// Approx overhead of rdtsc
const RDTSC_SPEED: u64 = 60;

struct Benchmark {
    /// Backing memory for benchmarks
    storage: *mut u8,

    /// Results
    tmp: Vec<u64>,
}

impl Benchmark {
    fn new() -> Self {
        let ret = Self {
            storage: unsafe {
                std::alloc::alloc(
                    Layout::from_size_align(MAX_BENCH_SIZE, 4096).unwrap())
            },
            tmp: Vec::with_capacity(MAX_BENCH_SIZE / 64),
        };

        // Make sure all the memory is paged in
        unsafe {
            for ii in 0..MAX_BENCH_SIZE {
                ret.storage.offset(ii as isize).write_volatile(5);
            }
        }

        ret
    }

    fn benchmark(&mut self, chunk_size: usize) -> Option<(usize, u64)> {
        if chunk_size == 0 || MAX_BENCH_SIZE == 0 ||
                chunk_size % 64 != 0 || MAX_BENCH_SIZE % 64 != 0 ||
                chunk_size > MAX_BENCH_SIZE {
            return None;
        }

        // Find the size of this benchmark
        // We must always at least write 16 chunks, and we must write at least
        // 4 GiB
        let bench_size = (4 * 1024 * 1024 * 1024).max(chunk_size);

        // Compute number of bytes we're actually going to write
        let written = (bench_size / chunk_size) * chunk_size;

        // Benchmark enough times to write `TO_WRITE` bytes in `CHUNK_SIZE`
        // chunks
        self.tmp.clear();
        for _ in 0..bench_size / chunk_size {
            let it = rdtsc();

            let mut ptr = self.storage as usize;
            while ptr < self.storage as usize + chunk_size {
                unsafe {
                    std::arch::asm!(r#"
                        mov dword ptr [{ptr} + 0x00], {val:e}
                        mov dword ptr [{ptr} + 0x04], {val:e}
                        mov dword ptr [{ptr} + 0x08], {val:e}
                        mov dword ptr [{ptr} + 0x0c], {val:e}
                        mov dword ptr [{ptr} + 0x10], {val:e}
                        mov dword ptr [{ptr} + 0x14], {val:e}
                        mov dword ptr [{ptr} + 0x18], {val:e}
                        mov dword ptr [{ptr} + 0x1c], {val:e}
                        mov dword ptr [{ptr} + 0x20], {val:e}
                        mov dword ptr [{ptr} + 0x24], {val:e}
                        mov dword ptr [{ptr} + 0x28], {val:e}
                        mov dword ptr [{ptr} + 0x2c], {val:e}
                        mov dword ptr [{ptr} + 0x30], {val:e}
                        mov dword ptr [{ptr} + 0x34], {val:e}
                        mov dword ptr [{ptr} + 0x38], {val:e}
                        mov dword ptr [{ptr} + 0x3c], {val:e}
                    "#,
                        ptr = in(reg) ptr,
                        val = in(reg) 0x69696969u32,
                    );
                }

                ptr += 64;
            }

            self.tmp.push(rdtsc() - it - RDTSC_SPEED);
        }

        self.tmp.sort();

        Some((written, self.tmp[self.tmp.len() / 8]))
    }

    fn benchmark_stream(&mut self, chunk_size: usize) -> Option<(usize, u64)> {
        if chunk_size == 0 || MAX_BENCH_SIZE == 0 ||
                chunk_size % 64 != 0 || MAX_BENCH_SIZE % 64 != 0 ||
                chunk_size > MAX_BENCH_SIZE {
            return None;
        }

        // Find the size of this benchmark
        // We must always at least write 16 chunks, and we must write at least
        // 4 GiB
        let bench_size = (4 * 1024 * 1024 * 1024).max(chunk_size * 16);

        // Compute number of bytes we're actually going to write
        let written = (bench_size / chunk_size) * chunk_size;

        // Benchmark enough times to write `TO_WRITE` bytes in `CHUNK_SIZE`
        // chunks
        self.tmp.clear();
        for _ in 0..bench_size / chunk_size {
            let it = rdtsc();

            let mut ptr = self.storage as usize;
            while ptr < self.storage as usize + chunk_size {
                unsafe {
                    std::arch::asm!(r#"
                        movnti dword ptr [{ptr} + 0x00], {val:e}
                        movnti dword ptr [{ptr} + 0x04], {val:e}
                        movnti dword ptr [{ptr} + 0x08], {val:e}
                        movnti dword ptr [{ptr} + 0x0c], {val:e}
                        movnti dword ptr [{ptr} + 0x10], {val:e}
                        movnti dword ptr [{ptr} + 0x14], {val:e}
                        movnti dword ptr [{ptr} + 0x18], {val:e}
                        movnti dword ptr [{ptr} + 0x1c], {val:e}
                        movnti dword ptr [{ptr} + 0x20], {val:e}
                        movnti dword ptr [{ptr} + 0x24], {val:e}
                        movnti dword ptr [{ptr} + 0x28], {val:e}
                        movnti dword ptr [{ptr} + 0x2c], {val:e}
                        movnti dword ptr [{ptr} + 0x30], {val:e}
                        movnti dword ptr [{ptr} + 0x34], {val:e}
                        movnti dword ptr [{ptr} + 0x38], {val:e}
                        movnti dword ptr [{ptr} + 0x3c], {val:e}
                    "#,
                        ptr = in(reg) ptr,
                        val = in(reg) 0x69696969u32,
                    );
                }

                ptr += 64;
            }

            self.tmp.push(rdtsc() - it - RDTSC_SPEED);
        }

        self.tmp.sort();

        Some((written, self.tmp[self.tmp.len() / 8]))
    }
}

fn main() {
    let mut bench = Benchmark::new();

    // Benchmark RDTSC
    let mut min_rdtsc = !0;
    for _ in 0..100000 {
        let it = rdtsc();
        min_rdtsc = min_rdtsc.min(rdtsc() - it);
    }

    let rdtsc_speed = min_rdtsc;
    eprintln!("Calibrated RDTSC to approx {}", rdtsc_speed);
    assert!(RDTSC_SPEED == rdtsc_speed, "Update the RDTSC_SPEED");

    let mut chunk_size = 64;
    while chunk_size <= MAX_BENCH_SIZE {
        if let (Some((_, cycles)), Some((_, cycles_stream))) = (
            bench.benchmark(chunk_size),
            bench.benchmark_stream(chunk_size),
        ) {
            // I have a 2.1 GHz processor, you should probably change this
            // if you do not :) :) :) :)
            let time    = cycles        as f64 / 2.1e9;
            let time_st = cycles_stream as f64 / 2.1e9;

            // Written in gigs
            let wg = chunk_size as f64 / 1024. / 1024. / 1024.;

            println!("{:15} {:20.9} {:20.9}",
                chunk_size, wg / time, wg / time_st);
        }

        chunk_size =
            (chunk_size + 64).max((chunk_size as f64 * 1.01) as usize);
        chunk_size = chunk_size & !0x3f;
    }
}

