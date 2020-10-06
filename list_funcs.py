import re
comments = re.compile(r"///(.*)", re.MULTILINE)
signature = re.compile(r"(?:///.*\s)*\s*pub\sfn\s[^p][^r][^e][^{]+",
                       re.MULTILINE)
name_n_return = re.compile(r"fn\s+[^{]+", re.MULTILINE)
func_name = re.compile(r"fn\s+([a-z A-Z] [a-z A-Z 0-9 _]*|_ [a-z A-Z 0-9 _]+)")
comment_lines = []
func_desc = []
print("User functions defined are listed below:\n")
with open("./src/server/func_man/functions.rs") as f:
    for i in re.findall(signature, f.read()):
        comment_lines = re.findall(comments, i)
        sign = re.findall(name_n_return, i)
        desc = ""
        for j in comment_lines:
            desc += " ".join(j.split())
            desc += "\n"
        func_desc += [desc]
        print("Signature: ", sign[0])
        print("Description: ", desc)
