import re
reobj = re.compile(r"pub\sfn [^{]+ {", re.MULTILINE)
with open("./src/server/func_man/functions.rs") as f:
    for i in re.findall(reobj, f.read()):
        print(i)
