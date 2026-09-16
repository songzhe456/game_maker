import subprocess
def run():
    build = subprocess.run(["cargo","build","--release"],capture_output=True)
    if build.returncode == 0:
        print(f"构建成功")
    else:
        print(f"构建失败:{str(build.stderr)}")
if __name__ == "__main__":
    run()