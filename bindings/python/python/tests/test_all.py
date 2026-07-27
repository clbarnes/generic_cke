from .common import FORMAT, SEPARATOR


def test_import():
    from generic_cke_py import GenericChunkKeyEncoding  # noqa


def test_sum_as_string():
    from generic_cke_py import GenericChunkKeyEncoding

    cke = GenericChunkKeyEncoding(FORMAT, SEPARATOR)
    encoded = cke.encode_chunk_key((0, 1, 2, 3, 4, 5, 6))
    assert encoded == "potato0-2/004_1:3:5,6"


def test_from_dict():
    from generic_cke_py import GenericChunkKeyEncoding

    cke = GenericChunkKeyEncoding.from_dict(
        {
            "name": "generic",
            "configuration": {
                "format": FORMAT,
                "separator": SEPARATOR,
            },
        }
    )
    assert type(cke) is GenericChunkKeyEncoding
    assert cke.format == FORMAT
    assert cke.separator == SEPARATOR


def test_to_dict():
    from generic_cke_py import GenericChunkKeyEncoding

    cke = GenericChunkKeyEncoding(FORMAT, SEPARATOR)
    d = cke.to_dict()
    assert d == {
        "name": "generic",
        "configuration": {
            "format": FORMAT,
            "separator": SEPARATOR,
        },
    }
