# ファイル名を指定
afile = "./fizzbuzz_python.txt"
bfile = "./fizzbuzz_rust.txt"

# ファイルを文字列として読み込む
with open(afile, "r") as fp:
    astr = fp.read()
with open(bfile, "r") as fp:
    bstr = fp.read()

# 念のため余分な空行をトリム
astr = astr.strip()
bstr = bstr.strip()

# 比較 ---
if astr == bstr:
    print("OK")
else:
    print("NG")

