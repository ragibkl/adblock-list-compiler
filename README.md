# adblock-list-compiler

CLI tool to compile multiple adblock list sources into a single adblock list file

# Introduction

An adblock dns server is a DNS server that resolves most domain names to ip address as usual, but also blocks known domain for ads.

In order to function properly, an adblock DNS server needs an adblock list, which is basically a list of domains blacklist that it should block.
Depending on what service is used to run the DNS server, the adblock list file needs to be in a specific format.

Online, there are plenty of projects dedicated to maintaining freely available adblock list.
While it is entirely possible to use just a single adblock list file online, most of the time we want to compile our own custom adblock list by combining multiple blacklist sources based on our individual needs.
The online adblock list might also be written in specific formats, that we need to combine and adapt to the right format that we can use.

This is where this project comes in.
This project helps you fetch adblock list from multiple sources, parses them, compiles them into a single list, deduplicates, and writes it into a single file with the right format.
At the moment, only bind zone output files are supported, with plans to add support for other file formats.

This project was created mainly to support compiling adblock list of my [Adblock DNS Server](https://github.com/ragibkl/adblock-dns-server) project, but you are free to use it in any other Adblock DNS project as you see fit.

# Getting Started

## Installation

You will need to have `rust` and `cargo` installed on your system.
Head over to rustup.sh to get that done.

Install the project with `cargo`:
```
cargo install adblock-list-compiler
```

Check the version you have
```
ablc -v

# output
adblock-list-compiler 0.0.4
```

## Configuration File

In order to start compiling and adblock list, you need to start with a valid configuration yaml file.
To get started quickly, you can refer to the Adblock DNS Server example [here](https://github.com/ragibkl/adblock-dns-server/blob/master/data/configuration.yaml).

A typical file should look as follows:
```yaml
# file: configuration.yaml

blacklist:
  - format: hosts
    path: https://sebsauvage.net/hosts/hosts
  - format: hosts
    path: https://raw.githubusercontent.com/r-a-y/mobile-hosts/master/AdguardDNS.txt
  - format: domains
    path: https://gitlab.com/quidsup/notrack-blocklists/raw/master/notrack-blocklist.txt
  - format: hosts
    path: ./blacklist.d/ads_custom.hosts

whitelist:
  - format: domains
    path: ./whitelist.d/blogspot_whitelist.txt
  - format: domains
    path: ./whitelist.d/facebook_whitelist.txt
  - format: domains
    path: https://raw.githubusercontent.com/raghavdua1995/DNSlock-PiHole-whitelist/master/whitelist.list

overrides:
  - format: cname
    path: ./overrides.d/bing-safesearch.zone
  - format: cname
    path: ./overrides.d/duckduckgo-safesearch.zone
  - format: cname
    path: ./overrides.d/google-safesearch.zone
  - format: cname
    path: ./overrides.d/ignore-whitelist.zone
```

The value for `path` can support both local file and web locations, and can also support relative paths.
Some adblock list online are published using either plain domain list, or hosts file.
This tool can adapt to both formats.
The blacklist are combined together and deduplicated.
The whitelist is used to exclude any legit domains that were somehow added to any of the blacklist.
The overrides are used mainly to rewrite some domains to an alternative hosts, mainly used to adapt forced safe search on some supported search engines.

## Validating the config file

Run the following to make sure the config file can be parsed correctly:
```
ablc config check -c /path/to/configuration.yaml
```

It should say the file can be parsed correctly, and print out some debug messages of the configuration.

## Compiling the adblock list

Run the following to compile your adblock list:
```
# Compile with your own configuration file
ablc compile -c /path/to/configuration.yaml -o blacklist.zone -f zone

# Try the compile with an upstream configuration file from the Adblock DNS Server project!
ablc compile -c https://raw.githubusercontent.com/ragibkl/adblock-dns-server/master/data/configuration.yaml -o blacklist.zone -f zone
```
