import subprocess

subprocess.Popen(["cargo","build","--release"]).communicate()