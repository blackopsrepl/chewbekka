# Changelog

All notable changes to this project will be documented in this file. See [commit-and-tag-version](https://github.com/absolute-version/commit-and-tag-version) for commit guidelines.

## [1.2.0](///compare/v1.1.0...v1.2.0) (2025-03-27)


### Bug Fixes

* **extract.rs:** added support for both files and directories as argument f1e1144

## 1.1.0 (2025-03-18)


### Features

* add base application 6702bdb
* add installer scripts 0552a46
* add task splitter f20cb94
* **build:** add build script for repository a4826d3
* **build:** Add GitHub Actions workflow for Rust 6dc6159
* **build:** Add installation command to Makefile 0338d93
* **build:** Add Makefile and utility scripts for formatting, linting, and testing d2fb768
* **debloat:** Add debloat module and functions bdec22f
* **debloat:** Add debloating functionality e08b8b2
* **debloat:** Added debloat functionality to remove unnecessary lingo from markdown files 501e3cb
* **expand:** Add functionality to analyze markdown files and generate subtasks b5ba799
* **extract:** add Mutex to ensure thread safety 6daf074
* **extract:** Change return type to use Mutex<HashMap<String, String>> 5ee6ae5
* isolated `summarize` command into subcommand_summarize function 79de3a0
* **subcommand_summarize:** add mutex for safe thread access 389c924


### Bug Fixes

* fix target directory for util scripts 963ba90
* **main:** add async keyword to subcommand_summarize 9e54e1f
