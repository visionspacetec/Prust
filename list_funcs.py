import re
comments = re.compile(r"///(.*)", re.MULTILINE)
func_params = re.compile(r"fn[^{\"\->\"]+",re.MULTILINE)
signature = re.compile(r"(?:///.*\s)*\s*pub\sfn\s[^p][^r][^e][^{]+",
                       re.MULTILINE)
comment_lines = []
func_desc = []
with open("./src/server/func_man/test.rs") as f:
    for i in re.findall(signature, f.read()):
        print(i)
        comment_lines = re.findall(comments, i)
        desc = ""
        for j in comment_lines:
            desc += " ".join(j.split())
            desc += "\n"
        func_desc += [desc]
print(func_desc)
