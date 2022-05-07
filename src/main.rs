use std::mem::size_of;
use std::alloc::Layout;

fn rdtsc() -> u64 {
    unsafe { core::arch::x86_64::_rdtsc() }
}

/// Maximum benchmark size, in bytes
const MAX_BENCH_SIZE: usize = 1024 * 1024 * 1024;

struct Benchmark {
    /// Backing memory for benchmarks
    storage: *mut u8,
}

impl Benchmark {
    fn new() -> Self {
        let ret = Self {
            storage: unsafe {
                std::alloc::alloc(
                    Layout::from_size_align(MAX_BENCH_SIZE, 4096).unwrap())
            },
        };

        // Make sure all the memory is paged in
        unsafe {
            for ii in 0..MAX_BENCH_SIZE {
                ret.storage.offset(ii as isize).write_volatile(5);
            }
        }

        ret
    }

    fn benchmark<const TO_WRITE: usize, const CHUNK_SIZE: usize>(&mut self) {
        assert!(CHUNK_SIZE > 0 && CHUNK_SIZE % size_of::<i32>() == 0 &&
            TO_WRITE % size_of::<i32>() == 0 &&
            CHUNK_SIZE <= MAX_BENCH_SIZE);

        // Benchmark enough times to write `TO_WRITE` bytes in `CHUNK_SIZE`
        // chunks
        for _ in 0..TO_WRITE / CHUNK_SIZE {
            let mut ii = 0;
            while ii < CHUNK_SIZE {
                unsafe {
                    (self.storage.offset(ii as isize) as *mut i32)
                        .write_volatile(69i32);
                }

                ii += size_of::<u32>();
            }
        }

        // Write the remaining partial chunk, if there was a partial chunk
        let mut ii = 0;
        while ii < TO_WRITE % CHUNK_SIZE {
            unsafe {
                (self.storage.offset(ii as isize) as *mut i32)
                    .write_volatile(69i32);
            }

            ii += size_of::<u32>();
        }
    }

    fn benchmark_stream<const TO_WRITE: usize, const CHUNK_SIZE: usize>(
            &mut self) {
        assert!(CHUNK_SIZE > 0 && CHUNK_SIZE % size_of::<i32>() == 0 &&
            TO_WRITE % size_of::<i32>() == 0 &&
            CHUNK_SIZE <= MAX_BENCH_SIZE);

        // Benchmark enough times to write `TO_WRITE` bytes in `CHUNK_SIZE`
        // chunks
        for _ in 0..TO_WRITE / CHUNK_SIZE {
            let mut ii = 0;
            while ii < CHUNK_SIZE {
                unsafe {
                    std::arch::x86_64::_mm_stream_si32(
                        self.storage.offset(ii as isize) as *mut i32, 69i32);
                }

                ii += size_of::<u32>();
            }

            // Make sure we can't optimize out rewriting the chunk
            unsafe {
                std::arch::asm!("");
            }
        }

        // Write the remaining partial chunk, if there was a partial chunk
        let mut ii = 0;
        while ii < TO_WRITE % CHUNK_SIZE {
            unsafe {
                std::arch::x86_64::_mm_stream_si32(
                    self.storage.offset(ii as isize) as *mut i32, 69i32);
            }

            ii += size_of::<u32>();
        }

        // Make sure we can't optimize out rewriting the chunk
        unsafe {
            std::arch::asm!("");
        }
    }
}

fn main() {
    let mut bench = Benchmark::new();

    macro_rules! verynice {
        ($($chunk_size:expr),*$(,)?) => {
            $(
                let it = rdtsc();
                bench.benchmark::<MAX_BENCH_SIZE, $chunk_size>();
                let elapsed = rdtsc() - it;

                // I have a 2.1 GHz processor, you should probably change this
                // if you do not :) :) :) :)
                let elapsed_sec = elapsed as f64 / 2.1e9;

                let it = rdtsc();
                bench.benchmark_stream::<MAX_BENCH_SIZE, $chunk_size>();
                let elapsed = rdtsc() - it;

                // I have a 2.1 GHz processor, you should probably change this
                // if you do not :) :) :) :)
                let elapsed_sec_stream = elapsed as f64 / 2.1e9;

                println!("{:12} {:15.4} {:15.4}", $chunk_size,
                    MAX_BENCH_SIZE as f64 / elapsed_sec / 1024. / 1024. / 1024.,
                    MAX_BENCH_SIZE as f64 / elapsed_sec_stream / 1024. / 1024. / 1024.);
            )*
        }
    }

    verynice!(
        //8192,
        4usize,8usize,12usize,16usize,20usize,24usize,28usize,32usize,36usize,40usize,44usize,48usize,52usize,56usize,60usize,64usize,68usize,72usize,76usize,80usize,84usize,88usize,92usize,96usize,100usize,104usize,108usize,112usize,116usize,120usize,124usize,128usize,132usize,136usize,140usize,144usize,148usize,152usize,156usize,160usize,164usize,168usize,172usize,176usize,180usize,184usize,188usize,192usize,196usize,200usize,204usize,208usize,212usize,216usize,224usize,232usize,240usize,248usize,256usize,264usize,272usize,280usize,288usize,296usize,304usize,312usize,320usize,328usize,340usize,352usize,364usize,376usize,388usize,400usize,412usize,424usize,436usize,452usize,468usize,484usize,500usize,516usize,532usize,548usize,568usize,588usize,608usize,628usize,648usize,672usize,696usize,720usize,744usize,768usize,796usize,824usize,852usize,880usize,912usize,944usize,976usize,1012usize,1048usize,1084usize,1124usize,1164usize,1204usize,1248usize,1292usize,1340usize,1388usize,1436usize,1488usize,1540usize,1596usize,1652usize,1712usize,1772usize,1836usize,1904usize,1972usize,2044usize,2120usize,2196usize,2276usize,2360usize,2444usize,2532usize,2624usize,2720usize,2820usize,2924usize,3032usize,3144usize,3260usize,3380usize,3504usize,3632usize,3764usize,3904usize,4048usize,4196usize,4352usize,4512usize,4676usize,4848usize,5028usize,5212usize,5404usize,5604usize,5812usize,6028usize,6252usize,6484usize,6724usize,6972usize,7228usize,7496usize,7772usize,8060usize,8360usize,8668usize,8988usize,9320usize,9664usize,10020usize,10392usize,10776usize,11176usize,11592usize,12020usize,12464usize,12928usize,13408usize,13904usize,14420usize,14956usize,15512usize,16088usize,16684usize,17304usize,17948usize,18616usize,19308usize,20024usize,20768usize,21540usize,22340usize,23168usize,24028usize,24920usize,25844usize,26804usize,27800usize,28832usize,29904usize,31016usize,32168usize,33364usize,34604usize,35892usize,37228usize,38612usize,40048usize,41536usize,43080usize,44680usize,46340usize,48064usize,49852usize,51704usize,53628usize,55624usize,57692usize,59836usize,62060usize,64368usize,66764usize,69248usize,71824usize,74496usize,77268usize,80144usize,83124usize,86216usize,89424usize,92752usize,96204usize,99784usize,103496usize,107348usize,111344usize,115488usize,119784usize,124240usize,128864usize,133660usize,138636usize,143796usize,149148usize,154700usize,160456usize,166428usize,172620usize,179044usize,185708usize,192620usize,199788usize,207224usize,214936usize,222936usize,231232usize,239840usize,248768usize,258028usize,267632usize,277592usize,287924usize,298640usize,309756usize,321284usize,333244usize,345648usize,358516usize,371860usize,385700usize,400056usize,414948usize,430396usize,446416usize,463032usize,480268usize,498144usize,516688usize,535920usize,555868usize,576560usize,598024usize,620284usize,643376usize,667328usize,692168usize,717936usize,744664usize,772384usize,801136usize,830960usize,861892usize,893976usize,927256usize,961776usize,997580usize,1034716usize,1073236usize,1113188usize,1154628usize,1197612usize,1242196usize,1288440usize,1336404usize,1386156usize,1437760usize,1491284usize,1546800usize,1604384usize,1664112usize,1726064usize,1790320usize,1856968usize,1926100usize,1997804usize,2072180usize,2149324usize,2229340usize,2312332usize,2398416usize,2487704usize,2580316usize,2676376usize,2776012usize,2879360usize,2986552usize,3097736usize,3213060usize,3332676usize,3456748usize,3585436usize,3718916usize,3857364usize,4000968usize,4149916usize,4304412usize,4464660usize,4630872usize,4803272usize,4982092usize,5167568usize,5359948usize,5559492usize,5766464usize,5981140usize,6203808usize,6434768usize,6674324usize,6922800usize,7180528usize,7447848usize,7725120usize,8012716usize,8311020usize,8620428usize,8941356usize,9274232usize,9619500usize,9977620usize,10349076usize,10734360usize,11133988usize,11548492usize,11978428usize,12424372usize,12886916usize,13366680usize,13864304usize,14380456usize,14915824usize,15471124usize,16047096usize,16644512usize,17264168usize,17906892usize,18573544usize,19265016usize,19982232usize,20726148usize,21497760usize,22298096usize,23128228usize,23989264usize,24882356usize,25808696usize,26769524usize,27766124usize,28799824usize,29872008usize,30984108usize,32137612usize,33334060usize,34575048usize,35862240usize,37197352usize,38582168usize,40018540usize,41508388usize,43053700usize,44656540usize,46319052usize,48043460usize,49832064usize,51687256usize,53611516usize,55607416usize,57677620usize,59824896usize,62052112usize,64362244usize,66758380usize,69243724usize,71821592usize,74495432usize,77268816usize,80145452usize,83129180usize,86223988usize,89434016usize,92763548usize,96217036usize,99799092usize,103514504usize,107368240usize,111365444usize,115511460usize,119811828usize,124272296usize,128898820usize,133697584usize,
    );
}

