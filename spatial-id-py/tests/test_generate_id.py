from spatial_id_py import generate_spatial_id

def test_generate_spatial_id():
    assert generate_spatial_id(0.0, 0.0, 10.0, 25).startswith("/25/")
    assert isinstance(generate_spatial_id(0.0, 0.0, 10.0, 25), str)
    print(generate_spatial_id(0.0, 0.0, 10.0, 25))
