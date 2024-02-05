from PIL import Image
import requests
from io import BytesIO
import sys
import os
import json
import re

def has_alpha_channel(image_path):
    with Image.open(image_path) as img:
        bands = img.getbands()
        return 'A' in bands

def rgb_to_hex(r, g, b):
    return f'#{r:02x}{g:02x}{b:02x}'.upper()

def camel_to_snake(camel_case_string):
    # Use a regular expression to insert underscores before capital letters
    snake_case_string = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', camel_case_string)
    # Convert the result to lowercase
    return snake_case_string.lower()

def get_pixel_color(image_path, x, y):
    # Open the image
    img = Image.open(image_path)

    # Get the color of the specified pixel
    pixel_color = img.getpixel((x, y))

    return pixel_color

def resize_online_image(image_path: str, temp_path) -> str | None:
    # Image is a URL, download it
    response = requests.get(image_path)
    if response.status_code == 200:
        # Read the image from the response content
        with Image.open(BytesIO(response.content)) as img:
            if 'A' in img.getbands():
                img = img.convert('RGBA')
            # Resize the image to fit image dimensions from args
            resized_img = img.resize(image_dimensions, Image.Resampling.LANCZOS)
            # Use a temporary local file to simulate a downloaded image
            resized_img.save(temp_path)

            return temp_path
    else:
        print(f"Failed to download image from URL: {image_path}")
        sys.exit(1)

def sort_args_values() -> (list[int], str, list[int]):
    """ When receiving arguments from rust sometimes it is 
    needed to strip quotes around string via the strip('"') """
    received_args = json.loads(sys.argv[1])
    d = { camel_to_snake(key): received_args[key] for key in received_args.keys()}
    sorted_keys = list(d.keys())
    sorted_keys.sort()
    sorted_values = tuple([d[key] for key in sorted_keys])

    return sorted_values

[image_dimensions, image_path, relative_coordinates] = sort_args_values()
image_path = image_path.strip('"')
temp_path = "temp_image.jpg"

resize_online_image(image_path, temp_path)
rgb = get_pixel_color(temp_path, *relative_coordinates)
hex = rgb_to_hex(*rgb)

if has_alpha_channel(temp_path):
    rgb.append(255)

# Pass to Rust
print(hex)
print(rgb)

# After the image has been processed, remove the temp file
os.remove(temp_path)
