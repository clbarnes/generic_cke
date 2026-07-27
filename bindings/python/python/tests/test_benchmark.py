from generic_cke_py import GenericChunkKeyEncoding
from .common import FORMAT, SEPARATOR


def test_bench_instantiation(benchmark):
    benchmark(GenericChunkKeyEncoding, FORMAT, SEPARATOR)


def test_bench_encode(benchmark):
    cke = GenericChunkKeyEncoding(FORMAT, SEPARATOR)
    chunk_idx = (0, 1, 2, 3, 4, 5, 6)
    benchmark(cke.encode_chunk_key, chunk_idx)
