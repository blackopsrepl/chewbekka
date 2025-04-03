# Changelog

All notable changes to this project will be documented in this file. See [commit-and-tag-version](https://github.com/absolute-version/commit-and-tag-version) for commit guidelines.

## [1.3.5](///compare/v1.3.4...v1.3.5) (2025-04-03)


### Features

* **extract:** add support for multiple file extensions c68eb3e

## [1.3.4](///compare/v1.3.3...v1.3.4) (2025-04-03)


### Bug Fixes

* **prompts:** isolate prompts 40b1dc8

## [1.3.3](///compare/v1.3.2...v1.3.3) (2025-03-30)


### Bug Fixes

* **Makefile:** fix help and phony for patch da2e706

## [1.3.2](///compare/v1.3.1...v1.3.2) (2025-03-29)


### Features

* Misc fixes 5f3c466

## [1.3.2](///compare/v1.3.1...v1.3.2) (2025-03-29)

## [1.3.1](///compare/v1.3.0...v1.3.1) (2025-03-28)

## [1.3.0](///compare/v1.2.0...v1.3.0) (2025-03-28)


### Features

* **debloat:** Refactor dissect_subtlety function to debloat ff9c83e


### Bug Fixes

* fix test for chat_completion, fix mutex guard over await e25d32b

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
