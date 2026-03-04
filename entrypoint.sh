#!/bin/bash

valgrind --leak-check=full cargo test
valgrind --tool=helgrind cargo test
