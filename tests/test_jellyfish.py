import csv
import platform
import pytest
import jellything


def test_rust_function():
    assert jellything.rust_function("hello") == "Hello, world! hello"