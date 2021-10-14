# CobbleDB

A LSM-based Key-Value Store in Rust

# Motivation

There is no open-source LSM-based key-value store in Rust natively. Some crates are either a wrapper to Google's LevelDB or Facebook's RocksDB,
or not support full functionality of LevelDB or RocksDB. Rust is becoming more and more popular in Database, Blockchain, etc.
A rust-native implementation of LSM is in need for a lot of projects.

# Goal

Good performance
- Draw experience of around a decade's research in write-optimized data structures (LSM or Be-tree)
- Take range query performance in conderataion by design
- Aim for fast storage devices (NVMe)

Verified features
- Take advantage of the rapid advacement of verification techniques to implement provable features (crash consitency, liveness, etc) 

# Reference

[1] Using lightweight formal methods to validate a key-value storage node in Amazon S3
[2] https://github.com/facebookexperimental/MIRAI
[3] https://github.com/diem/diem/blob/ed8731c12a318ce81ec241308bc9b4d7ab2a0241/language/tools/mirai-dataflow-analysis/README.md
