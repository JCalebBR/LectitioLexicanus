import subprocess
import sys
args = " ".join(sys.argv[1:])
subprocess.call(f"kindlegen.exe {args}", shell=True)
sys.exit(0)