import pytest
import maturin_musl_fail

def test_sum():
    assert maturin_musl_fail.sum_as_string(1,2) == '3'