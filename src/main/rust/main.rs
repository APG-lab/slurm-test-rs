
use std::cell;
use std::thread;
use clap::{Parser, Subcommand};

#[derive(Default)]
#[repr(align(4096))]
struct Page
{
    next: cell::Cell<Option<&'static Page>>,
    prev: cell::Cell<Option<&'static Page>>,
}

pub fn run_cpu (ncpu: usize)
{
    let ncpu_range = 1..=10;
    if ncpu_range.contains (&ncpu)
    {
        let handles = (0..ncpu).fold (Vec::new (), |mut acc, _| {
                acc.push (thread::spawn (|| {
                    let mut x: f64 = 0.0;
                    loop {
                        x = (x + 1.0).sin ();
                    }
                }));
                acc
            });

        for jh in handles.into_iter ()
        {
            jh.join ().unwrap ();
        }
    }
    else
    {
        eprintln! ("ncpu '{}' out of range {}..={}", ncpu, ncpu_range.start (), ncpu_range.end ());
    }
}

pub fn run_mem (mem: usize)
{
    let mem_range = 1..=10000;
    if mem_range.contains (&mem)
    {
        let head = &*Box::leak(Box::new(Page::default()));
        let mut tail = head;

        for memory_usage in (2u64 * 4096..).step_by(4096)
        {
            let memory_usage_mb = memory_usage / u64::pow (1000, 2);
            if memory_usage_mb as u64 > mem as u64 { break };

            let next = &*Box::leak(Box::new(Page::default()));
            tail.next.set(Some(next));
            next.prev.set(Some(tail));
            tail = next;

            if memory_usage % (16 << 20) == 0
            {
                let memory_usage_gb_div = u128::from (memory_usage) * 100 / u128::pow (1000, 3);
                let memory_usage_gb_frac = memory_usage_gb_div % 100;
                let memory_usage_gb_rest = memory_usage_gb_div / 100;

                println! ("{memory_usage} bytes {memory_usage_mb} Mb {memory_usage_gb_rest}.{memory_usage_gb_frac:#02} Gb");
            }
        }
    }
    else
    {
        eprintln! ("mem '{}' out of range {}..={}", mem, mem_range.start (), mem_range.end ());
    }
}

#[derive(Subcommand)]
enum Commands
{
    Mem {
        mem: usize,
    },
    CPU {
        ncpu: usize,
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli
{
    #[command(subcommand)]
    command: Commands,
}

fn main () {
    println! ("hello from slurm test");
    let cli = Cli::parse ();

    match cli.command
    {
        Commands::Mem { mem } => run_mem (mem),
        Commands::CPU { ncpu } => run_cpu (ncpu),
    }
}
