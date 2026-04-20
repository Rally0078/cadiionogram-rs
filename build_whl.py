import subprocess
import sys
import os

def run():
    print("Building rust project in release mode...")
    subprocess.run(["cargo", "build", "--release"], check=True)
    
    print("Generating python wheel...")
    subprocess.run([sys.executable, "setup.py", "bdist_wheel"], check=True)
    
    print("\nSuccess! Wheel generated in rust/dist/")

if __name__ == "__main__":
    # Change dir to script location
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    run()
