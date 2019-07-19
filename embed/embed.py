#!/usr/bin/env python

from ctypes import cdll

lib = cdll.LoadLibrary("target/release/libembed.so")
lib.process()
print("Done!")
