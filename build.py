import subprocess
try:
    subprocess.Popen(["cargo","build","--release"]).communicate()
    print("构建成功")
except Exception as e:
    print(f"构建失败:{e}")