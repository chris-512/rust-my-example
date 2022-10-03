#!/bin/sh

rustc executable.rs --extern rary=library.rlib --edition=2018 && ./executable