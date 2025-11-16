# slurm-test-rs
Uses the requested number of cpus and progressively allocates the requested memory to test job scheduler limits.

```bash
# To build the binary
cargo build --release
```


#### Usage
You can use this program to test the limits that the job schdeduler places on your jobs. The following examples use the SLURM scheduler. The slurm-test binary should be on your path.
```bash
slurm-test 
# hello from slurm test
# Usage: slurm-test <COMMAND>
#
# Commands:
#   mem   
#  cpu   
#  help  Print this message or the help of the given subcommand(s)
#
# Options:
#  -h, --help     Print help
#  -V, --version  Print version

```
## Testing cpu usage

```bash

# so the following doesn't actaully get cancelled but it does get limited to 200 % cpu
# each thread then uses 20 %
sbatch --output slurm-%j.out --error slurm-%j.err --cpus-per-task 2 --wrap "slurm-test cpu 10"

```
You can use some commands to view the cpu usage.
```bash
# first find the process id (pid) of the slurm-test program that you just submitted as a batch job
# the following displays the pid of all processes that belong to you
# note that we add f so that the name of the process is not truncated
ps -fU ${USER}
# to see all proceses belong to root, you could do
# ps -U root
# we can also use grep so that we can easily find slurm-test
ps -fU clayton | grep slurm-test
# clayton  1463778 2737628  0 09:27 pts/8    00:00:00 grep slurm-test
# clayton  2789600 2789599 99 09:23 ?        00:07:17 slurm-test cpu 10

# Now we know our pid (make sure you pick slurm test not grep)
# we can use htop to see how many cpus slurm-test is using
htop -p 2789600
#
#     PID▽USER      PRI  NI  VIRT   RES   SHR S CPU% MEM%   TIME+  Command
# 2789610 clayton    20   0  663M  1028   900 R 19.7  0.0  0:31.83 slurm-test cpu 10
# 2789609 clayton    20   0  663M  1028   900 R 19.7  0.0  0:31.99 slurm-test cpu 10
# 2789608 clayton    20   0  663M  1028   900 R 20.2  0.0  0:32.02 slurm-test cpu 10
# 2789607 clayton    20   0  663M  1028   900 R 20.2  0.0  0:31.95 slurm-test cpu 10
# 2789606 clayton    20   0  663M  1028   900 R 20.2  0.0  0:31.91 slurm-test cpu 10
# 2789605 clayton    20   0  663M  1028   900 R 20.2  0.0  0:31.93 slurm-test cpu 10
# 2789604 clayton    20   0  663M  1028   900 R 19.7  0.0  0:32.02 slurm-test cpu 10
# 2789603 clayton    20   0  663M  1028   900 R 20.2  0.0  0:31.81 slurm-test cpu 10
# 2789602 clayton    20   0  663M  1028   900 R 20.2  0.0  0:31.89 slurm-test cpu 10
# 2789601 clayton    20   0  663M  1028   900 R 20.2  0.0  0:31.91 slurm-test cpu 10
# 2789600 clayton    20   0  663M  1028   900 S 200.  0.0  5:19.36 slurm-test cpu 10

# The above shows that our program has 10 threads (so could potentially use 10 cpus)
# but each thread is only using 20 % of a cpu
# The parent pid (2789600, which we gave to htop) shows the total used by our program
# In total we use 200 %, so 2 cpus, which happens to be the number of cpus we requested.

# we can repeat this while requesting 10 cpus from slurm
sbatch --output slurm-%j.out --error slurm-%j.err --cpus-per-task 10 --wrap "slurm-test cpu 10"
htop -p 1755698
#
#     PID▽USER      PRI  NI  VIRT   RES   SHR S CPU% MEM%   TIME+  Command
# 1755708 clayton    20   0  663M  1024   900 R 100.  0.0  0:43.69 slurm-test cpu 10
# 1755707 clayton    20   0  663M  1024   900 R 100.  0.0  0:45.56 slurm-test cpu 10
# 1755706 clayton    20   0  663M  1024   900 R 100.  0.0  0:45.38 slurm-test cpu 10
# 1755705 clayton    20   0  663M  1024   900 R 100.  0.0  0:43.59 slurm-test cpu 10
# 1755704 clayton    20   0  663M  1024   900 R 100.  0.0  0:44.03 slurm-test cpu 10
# 1755703 clayton    20   0  663M  1024   900 R 100.  0.0  0:46.00 slurm-test cpu 10
# 1755702 clayton    20   0  663M  1024   900 R 100.  0.0  0:43.78 slurm-test cpu 10
# 1755701 clayton    20   0  663M  1024   900 R 100.  0.0  0:45.61 slurm-test cpu 10
# 1755700 clayton    20   0  663M  1024   900 R 100.  0.0  0:46.00 slurm-test cpu 10
# 1755699 clayton    20   0  663M  1024   900 R 100.  0.0  0:45.99 slurm-test cpu 10
# 1755698 clayton    20   0  663M  1024   900 S 1002  0.0  7:29.70 slurm-test cpu 10

# The above shows that all 10 cpus are used. As we only have 10 threads our
# program will only use 10 cpus. Requesting more from slurm will not make
# our program faster.

```

## Testing memory usage

Instead of allowing our program to run (but with limits) slurm will kill our job if our program requests too much memory.

```bash
# The test program actually allocates twice as much as it reports (I should fix this)
# Here we request just over 200 Mb of memory from slurm and ask our program to use
# 100 Mb (which will acutually use 200 Mb)
sbatch --output slurm-%j.out --error slurm-%j.err --mem 210m --wrap "slurm-test mem 100"
# Submitted batch job 28253

cat slurm-28253.out 
# hello from slurm test
# 16777216 bytes 16 Mb 0.01 Gb
# 33554432 bytes 33 Mb 0.03 Gb
# 50331648 bytes 50 Mb 0.05 Gb
# 67108864 bytes 67 Mb 0.06 Gb
# 83886080 bytes 83 Mb 0.08 Gb
# 100663296 bytes 100 Mb 0.10 Gb

cat slurm-28253.err

# The above shows that our program was able to use 100 Mb (really 200) of
# memory without any issues.
# Now lets try requesting less memory from slurm


sbatch --output slurm-%j.out --error slurm-%j.err --mem 200m --wrap "slurm-test mem 100"
# Submitted batch job 28911

cat slurm-28911.out
# hello from slurm test
# 16777216 bytes 16 Mb 0.01 Gb
# 33554432 bytes 33 Mb 0.03 Gb
# 50331648 bytes 50 Mb 0.05 Gb
# 67108864 bytes 67 Mb 0.06 Gb
# 83886080 bytes 83 Mb 0.08 Gb

# OOM is short for 'Out Of Memory'
cat slurm-28911.err
# Killed
# [2025-11-16T10:03:44.655] error: Detected 1 oom_kill event in StepId=28911.batch. Some of the step tasks have been OOM Killed.

# The above shows that our proram fails to use 100 Mb (really 200) and our job is killed

```

We can use the time program (not the bash builtin 'time' command) to see the maximum memory used by our process. This is useful when running a test job to see how much memory a new tool requires.

```bash
# 'time' is a bash builtin, you need the full path here '/usr/bin/time'
sbatch --output slurm-%j.out --error slurm-%j.err --mem 210m --wrap "/usr/bin/time -v slurm-test mem 100"
# Submitted batch job 29703

cat slurm-29703.err
#	Command being timed: "slurm-test mem 100"
#	User time (seconds): 0.00
#	System time (seconds): 0.11
#	Percent of CPU this job got: 95%
#	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.13
#	Average shared text size (kbytes): 0
#	Average unshared data size (kbytes): 0
#	Average stack size (kbytes): 0
#	Average total size (kbytes): 0
#	Maximum resident set size (kbytes): 199576
#	Average resident set size (kbytes): 0
#	Major (requiring I/O) page faults: 7
#	Minor (reclaiming a frame) page faults: 49422
#	Voluntary context switches: 7
#	Involuntary context switches: 3
#	Swaps: 0
#	File system inputs: 1752
#	File system outputs: 8
#	Socket messages sent: 0
#	Socket messages received: 0
#	Signals delivered: 0
#	Page size (bytes): 4096
#	Exit status: 0

# the above shows that the programs memory usage (Maximum resident set size) peaked at 199576 kilobytes or 199.58 Mb

```

