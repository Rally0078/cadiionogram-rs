import os
import shutil
from setuptools import setup, Distribution

class BinaryDistribution(Distribution):
    def has_ext_modules(foo):
        return True

def copy_built_lib():
    # Detect platform extension
    ext = ".dll" if os.name == "nt" else ".so"
    src = os.path.join("target", "release", "mdreader_rs" + ext)
    dst = "./dist/mdreader_rs.pyd" if os.name == "nt" else "./dist/mdreader_rs.so"
    
    if os.path.exists(src):
        print(f"Copying {src} to {dst}")
        shutil.copy(src, dst)
    else:
        print(f"Warning: {src} not found. Ensure cargo build --release has been run.")

copy_built_lib()

setup(
    name="mdreader_rs",
    version="0.1.0",
    description="CADI MDx binary file reader (Rust extension)",
    distclass=BinaryDistribution,
    # This ensures the .pyd file is included as package data
    # We leave packages empty if we just have a top-level module
    packages=[],
    data_files=[('', ['./dist/mdreader_rs.pyd' if os.name == 'nt' else './dist/mdreader_rs.so'])],
    zip_safe=False,
)
