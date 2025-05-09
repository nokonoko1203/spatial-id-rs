from spatial_id_py import generate_spatial_id

if __name__ == "__main__":
    lat, lon, alt, zoom = 0.0, 0.0, 10.0, 25
    spatial_id = generate_spatial_id(lat, lon, alt, zoom)
    print(f"Spatial ID: {spatial_id}")
